const copyButton = document.querySelector("[data-copy]");

if (copyButton) {
  copyButton.addEventListener("click", async () => {
    const text = copyButton.getAttribute("data-copy");
    if (!text) return;

    try {
      await navigator.clipboard.writeText(text);
      const previous = copyButton.textContent;
      copyButton.textContent = "Copied";
      copyButton.classList.add("is-copied");
      setTimeout(() => {
        copyButton.textContent = previous;
        copyButton.classList.remove("is-copied");
      }, 1400);
    } catch (_) {
      // Clipboard can fail on some browsers/pages; keep the page usable.
    }
  });
}

const observer = new IntersectionObserver(
  (entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        entry.target.classList.add("is-visible");
        observer.unobserve(entry.target);
      }
    });
  },
  { threshold: 0.1 }
);

document.querySelectorAll(".reveal").forEach((node) => observer.observe(node));

function formatStars(value) {
  if (value < 1000) return String(value);
  const rounded = Math.round((value / 1000) * 10) / 10;
  return `${rounded}k`;
}

const starsTarget = document.getElementById("github-stars");

if (starsTarget) {
  fetch("https://api.github.com/repos/penso/arbor")
    .then((response) => {
      if (!response.ok) throw new Error("request_failed");
      return response.json();
    })
    .then((data) => {
      if (typeof data.stargazers_count === "number") {
        starsTarget.textContent = formatStars(data.stargazers_count);
      } else {
        starsTarget.textContent = "Star";
      }
    })
    .catch(() => {
      starsTarget.textContent = "Star";
    });
}
