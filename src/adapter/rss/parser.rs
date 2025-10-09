use std::time::Duration;

use anyhow::Context;

const ATOM_NAMESPACE: &str = "http://www.w3.org/2005/Atom";
const CONTENT_NAMESPACE: &str = "http://purl.org/rss/1.0/modules/content/";
const ITUNES_NAMESPACE: &str = "http://www.itunes.com/dtds/podcast-1.0.dtd";

fn parse_bool<'a, 'input>(node: roxmltree::Node<'a, 'input>) -> anyhow::Result<Option<bool>> {
    let Some(txt) = node.children().find_map(|item| item.text()) else {
        return Ok(None);
    };
    txt.parse::<bool>()
        .context("unable to parse boolean")
        .map(Some)
}

fn parse_date<'a, 'input>(
    node: roxmltree::Node<'a, 'input>,
) -> anyhow::Result<Option<chrono::DateTime<chrono::Utc>>> {
    let Some(txt) = node.children().find_map(|item| item.text()) else {
        return Ok(None);
    };
    chrono::DateTime::parse_from_rfc2822(txt)
        .context("unable to parse date")
        .map(|date| Some(date.to_utc()))
}

fn parse_duration<'a, 'input>(
    node: roxmltree::Node<'a, 'input>,
) -> anyhow::Result<Option<Duration>> {
    let Some(txt) = node.children().find_map(|item| item.text()) else {
        return Ok(None);
    };
    let mut result: u64 = 0;
    for item in txt.split(':') {
        result = result * 60 + item.parse::<u64>()?;
    }
    Ok(Some(Duration::from_secs(result)))
}

fn parse_text<'a, 'input>(node: roxmltree::Node<'a, 'input>) -> Option<String> {
    node.children()
        .find_map(|item| item.text())
        .map(|value| String::from(value.trim()))
}

impl std::str::FromStr for super::Rss {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let doc = roxmltree::Document::parse(input).context("unable to parse document")?;

        let root = doc.root_element();
        if !root.has_tag_name("rss") {
            anyhow::bail!("expected rss root element")
        }

        let channels = root
            .children()
            .filter(|item| item.has_tag_name("channel"))
            .map(super::Channel::try_from)
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(super::Rss { channels })
    }
}

impl super::Channel {
    fn parse_image<'a, 'input>(&mut self, node: roxmltree::Node<'a, 'input>) -> anyhow::Result<()> {
        for child in node.children() {
            match child.tag_name().name() {
                "link" => {
                    self.image_link = parse_text(child);
                }
                "url" => {
                    self.image_url = parse_text(child);
                }
                "title" => {
                    self.image_title = parse_text(child);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn parse_atom_namespace<'a, 'input>(
        &mut self,
        node: roxmltree::Node<'a, 'input>,
    ) -> anyhow::Result<()> {
        match node.tag_name().name() {
            "link" => {
                self.atom_link_href = node.attribute("href").map(String::from);
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_itunes_owner<'a, 'input>(
        &mut self,
        node: roxmltree::Node<'a, 'input>,
    ) -> anyhow::Result<()> {
        for child in node.children() {
            match node.tag_name().name() {
                "name" => {
                    self.itunes_owner_name = parse_text(child);
                }
                "email" => {
                    self.itunes_owner_email = parse_text(child);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn parse_itunes_namespace<'a, 'input>(
        &mut self,
        node: roxmltree::Node<'a, 'input>,
    ) -> anyhow::Result<()> {
        match node.tag_name().name() {
            "author" => {
                self.itunes_author = parse_text(node);
            }
            "category" => {
                self.itunes_category = node.attribute("text").map(String::from);
            }
            "explicit" => {
                self.itunes_explicit = parse_bool(node)?;
            }
            "image" => {
                self.itunes_image_href = node.attribute("href").map(String::from);
            }
            "subtitle" => {
                self.itunes_subtitle = parse_text(node);
            }
            "summary" => {
                self.itunes_summary = parse_text(node);
            }
            "owner" => {
                self.parse_itunes_owner(node)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_no_namespace<'a, 'input>(
        &mut self,
        node: roxmltree::Node<'a, 'input>,
    ) -> anyhow::Result<()> {
        match node.tag_name().name() {
            "link" => {
                self.link = parse_text(node);
            }
            "title" => {
                self.title = parse_text(node);
            }
            "description" => {
                self.description = parse_text(node);
            }
            "image" => {
                self.parse_image(node)?;
            }
            "language" => {
                self.language = parse_text(node);
            }
            "pubDate" => {
                self.publication_date = parse_date(node)?;
            }
            "lastBuildDate" => {
                self.last_build_date = parse_date(node)?;
            }
            "managingEditor" => {
                self.managing_editor = parse_text(node);
            }
            "webMaster" => {
                self.web_master = parse_text(node);
            }
            "item" => {
                self.items.push(super::ChannelItem::try_from(node)?);
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_child<'a, 'input>(&mut self, node: roxmltree::Node<'a, 'input>) -> anyhow::Result<()> {
        match node.tag_name().namespace() {
            None => self.parse_no_namespace(node),
            Some(ATOM_NAMESPACE) => self.parse_atom_namespace(node),
            Some(ITUNES_NAMESPACE) => self.parse_itunes_namespace(node),
            Some(namespace) => {
                tracing::warn!(namespace, "unknown namespace");
                Ok(())
            }
        }
    }
}

impl<'a, 'b> TryFrom<roxmltree::Node<'a, 'b>> for super::Channel {
    type Error = anyhow::Error;

    fn try_from(node: roxmltree::Node<'a, 'b>) -> Result<Self, Self::Error> {
        let mut channel = super::Channel::default();
        for child in node.children() {
            if let Err(err) = channel.parse_child(child) {
                tracing::warn!(error = ?err, "unable to parse child node");
            }
        }
        Ok(channel)
    }
}

impl super::ChannelItem {
    fn parse_child<'a, 'input>(&mut self, node: roxmltree::Node<'a, 'input>) -> anyhow::Result<()> {
        match node.tag_name().namespace() {
            None => self.parse_no_namespace(node),
            Some(CONTENT_NAMESPACE) => self.parse_content_namespace(node),
            Some(ITUNES_NAMESPACE) => self.parse_itunes_namespace(node),
            Some(namespace) => {
                tracing::warn!(namespace, "unknown namespace");
                Ok(())
            }
        }
    }

    fn parse_content_namespace<'a, 'input>(
        &mut self,
        node: roxmltree::Node<'a, 'input>,
    ) -> anyhow::Result<()> {
        match node.tag_name().name() {
            "encoded" => {
                self.content_encoded = parse_text(node);
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_itunes_namespace<'a, 'input>(
        &mut self,
        node: roxmltree::Node<'a, 'input>,
    ) -> anyhow::Result<()> {
        match node.tag_name().name() {
            "duration" => {
                self.itunes_duration = parse_duration(node)?;
            }
            "summary" => {
                self.itunes_summary = parse_text(node);
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_no_namespace<'a, 'input>(
        &mut self,
        node: roxmltree::Node<'a, 'input>,
    ) -> anyhow::Result<()> {
        match node.tag_name().name() {
            "title" => {
                self.title = parse_text(node);
            }
            "description" => {
                self.description = parse_text(node);
            }
            "enclosure" => {
                self.parse_enclosure(node)?;
            }
            "link" => {
                self.link = parse_text(node);
            }
            "guid" => {
                self.guid = parse_text(node);
                self.guid_perma_link = node
                    .attribute("isPermaLink")
                    .and_then(|value| value.parse::<bool>().ok());
            }
            "pubDate" => {
                self.publication_date = parse_date(node)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_enclosure<'a, 'input>(
        &mut self,
        node: roxmltree::Node<'a, 'input>,
    ) -> anyhow::Result<()> {
        self.enclosure_url = node.attribute("url").map(String::from);
        self.enclosure_type = node.attribute("type").map(String::from);
        self.enclosure_length = node.attribute("length").and_then(|value| {
            value
                .parse::<u64>()
                .inspect_err(|err| tracing::warn!(error = ?err, "unable to parse enclosure length"))
                .ok()
        });
        Ok(())
    }
}

impl<'a, 'b> TryFrom<roxmltree::Node<'a, 'b>> for super::ChannelItem {
    type Error = anyhow::Error;

    fn try_from(node: roxmltree::Node<'a, 'b>) -> Result<Self, Self::Error> {
        let mut channel = super::ChannelItem::default();
        for child in node.children() {
            if let Err(err) = channel.parse_child(child) {
                tracing::warn!(error = ?err, "unable to parse child node");
            }
        }
        Ok(channel)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[test]
    fn should_parse_rustacean_station() {
        let input = include_str!("../../../assets/podcast/rustacean-station.rss");
        let rss = crate::adapter::rss::Rss::from_str(input).unwrap();
        assert_eq!(rss.channels.len(), 1);
        assert_eq!(rss.channels[0].title.as_ref().unwrap(), "Rustacean Station");
        assert_eq!(rss.channels[0].items.len(), 178);
        for item in rss.channels[0].items.iter() {
            assert!(item.link.is_some() || item.enclosure_url.is_some());
        }
    }

    #[test]
    fn should_parse_floss_weekly() {
        let input = include_str!("../../../assets/podcast/floss-weekly.rss");
        let rss = crate::adapter::rss::Rss::from_str(input).unwrap();
        assert_eq!(rss.channels.len(), 1);
        assert_eq!(rss.channels[0].title.as_ref().unwrap(), "FLOSS Weekly");
        assert_eq!(rss.channels[0].items.len(), 174);
        for item in rss.channels[0].items.iter() {
            assert!(item.link.is_some() || item.enclosure_url.is_some());
        }
    }

    #[test]
    fn should_parse_la_derniere() {
        let input = include_str!("../../../assets/podcast/la-derniere.xml");
        let rss = crate::adapter::rss::Rss::from_str(input).unwrap();
        assert_eq!(rss.channels.len(), 1);
        assert_eq!(rss.channels[0].title.as_ref().unwrap(), "La dernière");
        assert_eq!(rss.channels[0].items.len(), 410);
        for item in rss.channels[0].items.iter() {
            assert!(item.link.is_some() || item.enclosure_url.is_some());
        }
    }
}
