/** @jsxImportSource @emotion/react */
import React from "react";
import { keyframes } from "@emotion/react";
import type { Interpolation, Theme } from "@emotion/react";

export interface SpinnerProps extends React.HTMLAttributes<HTMLSpanElement> {
  /** The size of the spinner (width and height) in pixels */
  size?: string;
  css?: Interpolation<Theme>;
}

const rotation = keyframes`
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
`;

export function LoadingSpinner({ size = "16px", css: customCss, ...props }: SpinnerProps) {
  return (
    <span
      {...props}
      css={[
        {
          width: size,
          height: size,
          border: "3px solid var(--text-color2, #000)",
          borderBottomColor: "transparent",
          borderRadius: "50%",
          display: "inline-block",
          boxSizing: "border-box",
          animation: `${rotation} 800ms linear infinite`,
        },
        customCss,
      ]}
    />
  );
}
