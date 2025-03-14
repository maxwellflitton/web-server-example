import { css } from "@emotion/react";

export const LoadingPageContainer = css`
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100vw;
  height: 100vh;
  background-color: var(--color-light-grey);
`;

export const SpinnerContainer = css`
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
`;

export const PageContainer = css`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center; /* Uncommented */
  box-sizing: border-box; /* Include padding in the height */
  height: 100vh;
  width: 100vw;
  background-color: var(--color-light-grey);
`;

export const ContentWrapper = css`
  display: flex;
  flex-direction: column; /* stack them vertically */
  width: 50vw;
  max-height: 85vh;
  overflow-y: auto; /* allow scrolling if content grows */
  background-color: var(--color-white);
  border-radius: 16px;
  border: 1px solid #ccc;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 24px;
`;

export const ContentErrorWrapper = css`
  display: flex;
  flex-direction: column; /* stack them vertically */
  width: 40vw;
  height: 85vh;
  overflow-y: auto; /* allow scrolling if content grows */
  background-color: var(--color-white);
  border-radius: 16px;
  border: 1px solid #ccc;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 24px;
`;
