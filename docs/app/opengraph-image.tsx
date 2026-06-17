import { ImageResponse } from "next/og";

export const runtime = "edge";
export const alt = "code-lang — A general-purpose interpreted programming language written in Rust.";
export const size = { width: 1200, height: 630 };
export const contentType = "image/png";

export default function Image() {
  return new ImageResponse(
    (
      <div
        style={{
          background: "#09090b",
          width: "100%",
          height: "100%",
          display: "flex",
          flexDirection: "column",
          alignItems: "flex-start",
          justifyContent: "center",
          padding: "80px 96px",
          fontFamily: "monospace",
        }}
      >
        {/* Logo mark */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            width: 72,
            height: 72,
            borderRadius: 16,
            background: "#18181b",
            border: "1px solid #27272a",
            marginBottom: 48,
          }}
        >
          <span
            style={{
              fontSize: 32,
              fontWeight: 700,
              background: "linear-gradient(135deg, #818cf8, #22d3ee)",
              backgroundClip: "text",
              color: "transparent",
              letterSpacing: "-1px",
            }}
          >
            cl
          </span>
        </div>

        {/* Name */}
        <div
          style={{
            fontSize: 72,
            fontWeight: 700,
            color: "#fafafa",
            letterSpacing: "-2px",
            lineHeight: 1,
            marginBottom: 24,
          }}
        >
          code-lang
        </div>

        {/* Tagline */}
        <div
          style={{
            fontSize: 28,
            color: "#71717a",
            lineHeight: 1.5,
            maxWidth: 700,
          }}
        >
          A general-purpose interpreted programming language written in Rust.
        </div>

        {/* Accent bar */}
        <div
          style={{
            position: "absolute",
            bottom: 0,
            left: 0,
            right: 0,
            height: 4,
            background: "linear-gradient(90deg, #818cf8, #22d3ee)",
          }}
        />
      </div>
    ),
    { ...size },
  );
}
