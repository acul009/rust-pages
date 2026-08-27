document
  .querySelectorAll("[data-content][data-content-key]")
  .forEach((element) => {
    try {
      const encodedContent = atob(element.dataset.content);
      const encodedKey = atob(element.dataset.contentKey);
      const key = Uint8Array.from(encodedKey, (character) =>
        character.charCodeAt(0),
      );
      const content = Uint8Array.from(
        encodedContent,
        (character, index) => character.charCodeAt(0) ^ key[index % key.length],
      );

      element.innerHTML = new TextDecoder().decode(content);
    } catch (error) {
      console.error("Failed to decode browser-only content", error);
    }
  });
