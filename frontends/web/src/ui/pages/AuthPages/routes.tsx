import { ConfirmUserPage } from "./ConfirmUserPage/ConfirmUserPage";
import { CreateSuperUserPage } from "./CreateSuperUserPage/CreateSuperUserPage";
import { LoginPage } from "./LoginPage/LoginPage";
import { SuperAdminPanelPage } from "./SuperAdminPanelPage/SuperAdminPanelPage";
import { UserActionsPage } from "./UserActionsPage/UserActionsPage";


export const AuthPageRoutes = [
    {
        path: "/confirm-user/:uuid?",
        element: <ConfirmUserPage/>,
    },
    {
        path: "/create-super-user",
        element: <CreateSuperUserPage/>,
    },
    {
        path: "/login",
        element: <LoginPage/>,
    },
    {
        path: "/superadmin-panel",
        element: <SuperAdminPanelPage/>,
    },
    {
        path: "/user-actions/:userId?",
        element: <UserActionsPage/>,
    },
]
