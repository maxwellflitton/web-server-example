import { css } from "@emotion/react";


export const LoadingPageContainer = css`
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100vw;
  height: 100vh;
  background-color: var(--color-white);
`;

export const SpinnerContainer = css`
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
`;

export const PageContainer = css`
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100vw;
  height: 100vh;
  background-color: var(--color-light-grey);
`;

export const FormWrapper = css`
  overflow-y: auto;
  background-color: var(--color-white);
  border-radius: 16px;
  border: 1px solid;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  width: 40%
`;
