/**
 * Source: https://codepen.io/absolutedevelopment/pen/EpwVzN
 */
function syntaxHighlight(jsonString) {
  const jsonSanitized = jsonString
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
  return jsonSanitized.replace(
    /("(\\u[a-zA-Z0-9]{4}|\\[^u]|[^\\"])*"(\s*:)?|\b(true|false|null)\b|-?\d+(?:\.\d*)?(?:[eE][+\-]?\d+)?)/g,
    (match) => {
      let className = "number";
      if (/^"/.test(match)) {
        if (/:$/.test(match)) {
          className = "key";
          // Don't highlight the trailing colon after keys
          return (
            '<span class="' + className + '">' + match.slice(0, -1) + "</span>:"
          );
        } else {
          className = "string";
        }
      } else if (/true|false/.test(match)) {
        className = "boolean";
      } else if (/null/.test(match)) {
        className = "null";
      }
      return '<span class="' + className + '">' + match + "</span>";
    }
  );
}
