import { css } from "@emotion/react";

export const Container = css`
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background-color: var(--color-light-grey);
`;

export const FormWrapper = css`
  width: 35%;
  overflow-y: auto;
  background-color: var(--color-white);
  border-radius: 16px;
  border: 1px solid;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
`;
