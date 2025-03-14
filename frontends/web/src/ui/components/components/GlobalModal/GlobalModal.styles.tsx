import { css } from "@emotion/react";

export const overlayStyle = css`
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
`;

export const contentStyle = css`
  background-color: white;
  padding: 16px;
  border-radius: 8px;
  overflow-y: auto;
  position: relative;
`;
