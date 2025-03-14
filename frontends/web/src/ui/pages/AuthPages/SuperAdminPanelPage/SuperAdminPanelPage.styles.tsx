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

export const Container = css`
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100vw;
    min-height: 100vh;
    background-color: var(--color-light-grey);
`;

export const TableWrapper = css`
    width: 85vw;
    min-height: 50vh;
    max-height: 92vh;
    overflow-y: auto;
    background-color: var(--color-white);
    border-radius: 12px;
    padding: 32px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    // margin-top: 32px;
`;

export const HeaderSection = css`
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;

    h1 {
        margin: 0;
        font-size: 32px;
        font-weight: 600;
    }
`;

export const CreateUserFormWrapper = css`
  width: 35%;
  height: 80vh;
  overflow-y: auto;
  background-color: var(--color-white);
  border-radius: 16px;
  border: 1px solid;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
`;