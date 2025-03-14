import { css } from '@emotion/react';

export const overlay = css`
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 10;
  /* Translucent light grey overlay */
  background-color: var(--overlay-light-grey);
  display: flex;
  align-items: center;
  justify-content: center;
`;

export const SpinnerContainer = css`
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
`;
