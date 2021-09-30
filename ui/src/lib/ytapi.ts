export function load() {
  return new Promise<void>((resolve) => {
    if (!window.YT) {
      const el = document.createElement("script")!;
      el.src = "https://www.youtube.com/iframe_api";
      const firstScriptEl = document.getElementsByTagName("script")[0];
      firstScriptEl.parentNode?.insertBefore(el, firstScriptEl);

      // @ts-ignore
      window.onYouTubeIframeAPIReady = () => resolve();
    } else {
      resolve();
    }
  });
}
