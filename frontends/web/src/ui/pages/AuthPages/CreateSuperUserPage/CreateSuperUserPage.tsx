/** @jsxImportSource @emotion/react */
import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { CreateSuperUserForm } from "./CreateSuperUserForm/CreateSuperUserForm";
import { createSuperUser } from "#serverApi/auth/users/createSuperUser";
import { CreateSuperUserFormSchema } from "./CreateSuperUserForm/CreateSuperUserForm";
import { FormWrapper, Container } from "./CreateSuperUserPage.styles";


export const CreateSuperUserPage: React.FC = () => {
    const navigate = useNavigate();

    const [error, setError] = useState("");
    const [failure, setFailure] = useState(false);
    
    const submitCreateSuperUserForm = async (data: CreateSuperUserFormSchema) => {
        setError("");
        setFailure(false);
    
        try {
            const requestPayload: any = {
                email: data.email,
                password: data.password,
                username: data.username,
                first_name: data.firstName,
                last_name: data.lastName,
                user_role: "Super Admin"
            };
            const response = await createSuperUser(requestPayload);
        
            if (response.status === 201) {
                navigate("/login"); 
            } else if (response.status === 0) {
                console.log(response.body);
                setError(`Account creation failed due to an internet connection issue. Please check your internet and try again.`);
                setFailure(true);
            } else {
                console.log(response.body);
                setError(`Account creation failed. Please try again.`);
                setFailure(true);
            }
        } catch (err) {
            console.error(err);
            setError("An unexpected error occurred. Please try again.");
            setFailure(true);
        };
    };
    
    return (
        <div css={Container}>
            <div css={FormWrapper}>
                <CreateSuperUserForm
                    apiError={error}
                    apiFailure={failure}
                    submitCreateSuperUserForm={submitCreateSuperUserForm}
                />
            </div>
        </div>
    );
};