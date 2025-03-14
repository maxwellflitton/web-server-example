import { css } from "@emotion/react";

export const cardContainer = css`
  width: 100%;
`;

export const columnsContainer = css`
  display: flex;
  flex-direction: row;
  gap: 1rem;

  /* Stack columns on top of each other for narrower screens */
  @media (max-width: 768px) {
    flex-direction: column;
  }
`;

export const column = css`
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
`;

export const infoBox = css`
  border: 1px solid #ccc;
  border-radius: 12px;
  padding: 12px;
`;

export const infoTitle = css`
  font-size: 18px;
  font-weight: 600;
  color: #000;
  margin-bottom: 4px;
`;

export const infoContent = css`
  font-size: 14px;
  color: #666;
`;

export const roleItem = css`
  border: 1px solid #ccc;
  border-radius: 12px;
  margin-top: 8px;
  padding: 8px;
`;
