export const fetchJson = (url: string) => {
  fetch(url).then((res) => {
    if (res.ok) {
      return res.json();
    }
    throw res.json();
  });
};
