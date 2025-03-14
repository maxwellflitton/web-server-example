/** @jsxImportSource @emotion/react */
import React from "react";
import { passThroughRef } from "src/ui/helpers/passThroughRef";
import type { Interpolation, Theme } from "@emotion/react";

type ButtonProps = React.JSX.IntrinsicElements["button"] & {
    id?: string;
    css?: Interpolation<Theme>;
};

function _Button(props: ButtonProps) {
  return (
    <button
      {...props}
      id={["id", props.id].filter(Boolean).join(" ")}
      css={[
        {
          cursor: "pointer",
          border: "1px solid var(--gray)",
          padding: "8px 16px",
          borderRadius: "8px",
        },
        props.css,
      ]}
    />
  );
}

export const FormButton = passThroughRef((props: ButtonProps) => {
  return (
    <_Button
      {...props}
      css={[
        {
          background: "#000000",
          color: "#FFFFFF",
          "&:hover": {
            background: "#333333",
          },
          "&:disabled": {
              opacity: 0.5,
              cursor: "not-allowed",
          },
        },
        props.css,
      ]}
    />
  );
});



