use askama::Template;

pub trait User {
    fn login_url(&self) -> String;
    fn name(&self) -> &str;
}

#[derive(Debug, Template)]
#[template(path = "view/login.html")]
pub struct LoginView<U: User> {
    users: Vec<U>,
}

impl<U: User> LoginView<U> {
    pub fn new(users: Vec<U>) -> Self {
        Self { users }
    }
}

#[cfg(test)]
mod tests {
    use askama::DynTemplate;

    use super::LoginView;

    struct TestUser {
        index: u8,
        name: String,
    }

    impl TestUser {
        fn new(index: u8) -> Self {
            Self {
                index,
                name: format!("User {index}"),
            }
        }
    }

    impl super::User for TestUser {
        fn login_url(&self) -> String {
            format!("/api/login/{}", self.index)
        }

        fn name(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn should_display_list_of_users() {
        let view = LoginView::new((1..5).map(|index: u8| TestUser::new(index)).collect());
        let output = view.dyn_render().unwrap();
        assert!(output.contains("/api/login/1"));
        assert!(output.contains("User 1"));
    }
}
