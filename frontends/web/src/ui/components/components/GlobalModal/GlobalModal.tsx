/** @jsxImportSource @emotion/react */
import React from "react";
import { createPortal } from "react-dom";
import { css, Interpolation, Theme } from "@emotion/react";
import * as styles from "./GlobalModal.styles";

interface GlobalModalProps {
  isOpen: boolean;
  onRequestClose: () => void;
  children: React.ReactNode;
  contentCss?: Interpolation<Theme>;
  disableCloseModal?: boolean;
}

export function GlobalModal({
  isOpen,
  onRequestClose,
  children,
  contentCss,
  disableCloseModal = false,
}: GlobalModalProps) {
  if (!isOpen) return null;
  
  // We store the new element as a ref so the portal container element persists across renders
  // This means that a ref to the element within the opened modal will be in the DOM tree, and persistent
  const elRef = React.useRef(document.createElement("div"));
  const modalRoot = document.getElementById("modal-root")!;

  React.useEffect(() => {
    const el = elRef.current;
    modalRoot.appendChild(el);
    return () => {
      modalRoot.removeChild(el);
    };
  }, [modalRoot]);

  return createPortal(
    <div
      css={styles.overlayStyle}
      onClick={() => {
        if (!disableCloseModal) {
          onRequestClose();
        }
      }}
    >
      <div
        css={[styles.contentStyle, contentCss]}
        onClick={(e) => {
          e.stopPropagation();
        }}
      >
        {children}
      </div>
    </div>,
    elRef.current
  );
}
