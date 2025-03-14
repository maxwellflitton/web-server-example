/** @jsxImportSource @emotion/react */
import React, { useState } from "react";
import { css } from "@emotion/react";
import { GlobalModal } from "./GlobalModal";

export default {
  title: "Global/GlobalModal",
  component: GlobalModal,
};

export const Default = () => {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div style={{ padding: "24px" }}>
      <button onClick={() => setIsOpen(true)}>Open Modal</button>
      <GlobalModal isOpen={isOpen} onRequestClose={() => setIsOpen(false)}>
        <h2>This is a Portal Modal</h2>
        <p>You can put any content here.</p>
        <button onClick={() => setIsOpen(false)}>Close Modal</button>
      </GlobalModal>
    </div>
  );
};

export const WithCustomStyling = () => {
  const [isOpen, setIsOpen] = useState(false);

  // Define custom styles for the modal content using Emotion's css helper.
  const customContentStyles = css`
    max-width: 600px;
    padding: 32px;
    border: 3px solid #007bff;
    border-radius: 16px;
    background: linear-gradient(45deg, #f3ec78, #af4261);

    h2 {
      margin-top: 0;
      color: #fff;
    }

    p {
      color: #f5f5f5;
    }
  `;

  return (
    <div style={{ padding: "24px" }}>
      <button onClick={() => setIsOpen(true)}>Open Custom Styled Modal</button>
      <GlobalModal
        isOpen={isOpen}
        onRequestClose={() => setIsOpen(false)}
        contentCss={customContentStyles}
      >
        <h2>Custom Styled Modal</h2>
        <p>
          This modal has custom styles applied via the <code>contentCss</code> prop.
        </p>
        <button onClick={() => setIsOpen(false)}>Close Modal</button>
      </GlobalModal>
    </div>
  );
};

export const MultipleModals = () => {
  const [modalOneOpen, setModalOneOpen] = useState(false);
  const [modalTwoOpen, setModalTwoOpen] = useState(false);

  return (
    <div style={{ padding: "24px" }}>
      <div style={{ marginBottom: "16px" }}>
        <button onClick={() => setModalOneOpen(true)} style={{ marginRight: "8px" }}>
          Open Modal One
        </button>
        <button onClick={() => setModalTwoOpen(true)}>Open Modal Two</button>
      </div>
      <GlobalModal
        isOpen={modalOneOpen}
        onRequestClose={() => setModalOneOpen(false)}
      >
        <h2>Modal One</h2>
        <p>This is the first modal.</p>
        <button onClick={() => setModalOneOpen(false)}>Close Modal One</button>
      </GlobalModal>
      <GlobalModal
        isOpen={modalTwoOpen}
        onRequestClose={() => setModalTwoOpen(false)}
      >
        <h2>Modal Two</h2>
        <p>This is the second modal.</p>
        <button onClick={() => setModalTwoOpen(false)}>Close Modal Two</button>
      </GlobalModal>
    </div>
  );
};

/**
 * New story demonstrating disableCloseModal usage
 */
export const DisableCloseModal = () => {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div style={{ padding: "24px" }}>
      <button onClick={() => setIsOpen(true)}>
        Open Modal (overlay click disabled)
      </button>
      <GlobalModal
        isOpen={isOpen}
        onRequestClose={() => setIsOpen(false)}
        disableCloseModal
      >
        <h2>Modal With Overlay Click Disabled</h2>
        <p>
          You can only close this modal by clicking the "Close Modal" button.
          Clicking the backdrop won't close the modal.
        </p>
        <button onClick={() => setIsOpen(false)}>Close Modal</button>
      </GlobalModal>
    </div>
  );
};
