const fs = require("fs");

const manifest = fs.readFileSync(process.argv[3], "utf-8");

function formatBytes(bytes, decimals = 2) {
  if (bytes === 0) return "0 Bytes";
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + " " + sizes[i];
}

fs.writeFileSync(
  process.argv[5],
  fs.readFileSync(process.argv[4], "utf-8").replace(
    "<<" + process.argv[6] + ">>",
    "| Name | Size |\n|-|-|\n" +
      Object.entries(
        Object.fromEntries(
          fs
            .readdirSync(process.argv[2])
            .filter(
              (v) =>
                !(v.endsWith("rmeta") || v.endsWith("d") || v === "a.js") &&
                manifest.match(
                  "\\b" + v.replace(/(^lib|-[^-]*$)/g, "") + "\\b"
                ) &&
                v.includes(".")
            )
            .map((v) => [
              v.replace(/(^lib|-[^-]*$)/g, ""),
              fs.statSync(process.argv[2] + "/" + v).size,
            ])
        )
      )
        .map(([a, b]) => [a, formatBytes(b)])
        .map(([dep, size]) => `| ${dep} | ${size} |`)
        .join("\n")
  )
);
