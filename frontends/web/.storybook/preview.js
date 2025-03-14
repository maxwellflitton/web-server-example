import setupLocatorUI from "@locator/runtime";
import React from "react";
import { MemoryRouter } from "react-router-dom";
import "src/index.css";
import "src/tailwind.css";


if (process.env.NODE_ENV === "development") {
    setupLocatorUI();
}

/** @type { import('@storybook/react').Preview } */
const preview = {
    parameters: {
        actions: { argTypesRegex: "^on[A-Z].*" },
        controls: {
            matchers: {
                color: /(background|color)$/i,
                date: /Date$/i,
            },
        },
    },
};

function WaitForModalRoot({ children }) {
    const [modalRootAdded, setModalRootAdded] = React.useState(false);

    React.useEffect(() => {
        if (document.getElementById("modal-root")) {
            setModalRootAdded(true);
        }
    }, []);

    return (
        <>
            {/* Matches the modal-root div in the index.html file */}
            <div
                id="modal-root"
                ref={(e) => {
                    setModalRootAdded(!!e);
                }}
            >
                {modalRootAdded ? children : null}
            </div>
        </>
    );
}

export const decorators = [
    (Story, context) => {
        const useCustomRouter = context.parameters.useCustomRouter;

        const storyContent = (
            <WaitForModalRoot>
                <Story />
            </WaitForModalRoot>
        );

        // Wrap with MemoryRouter unless useCustomRouter is true. This allows
        // stories to use the router without needing to wrap the story content
        // in a MemoryRouter in each story file. However if useCustomRouter is
        // true, the story content will not be wrapped in a MemoryRouter, and 
        // the story file will be responsible for wrapping the content in 
        // a custom router. This is useful for stories that need to use a
        // specific route such as "/dashboard". 
        if (!useCustomRouter) {
            return (
                <div style={{ position: "absolute", left: 0, top: 0, minHeight: "100vh", width: "100vw" }}>
                    <MemoryRouter>
                        {storyContent}
                    </MemoryRouter>
                </div>
            );
        }

        // For stories specifying useCustomRouter, just render the story content without MemoryRouter
        return storyContent;
    },
];