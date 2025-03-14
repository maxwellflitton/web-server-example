/** @jsxImportSource @emotion/react */
import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import { LoginForm, LoginFormSchema } from "./LoginForm/LoginForm";
import { loginUser } from "#serverApi/auth/auth/login";
import { Container, FormWrapper } from "./LoginPage.styles";
import { setJwt, setRole } from "api-modules/auth/localStorage";

export const LoginPage: React.FC = () => {
  const navigate = useNavigate();
  const [error, setError] = useState("");
  const [failure, setFailure] = useState(false);

  const submitLoginForm = async (data: LoginFormSchema) => {
    setError("");
    setFailure(false);

    try {
      const requestPayload = {
        email: data.email,
        password: data.password,
        role: data.role,
      };
      const response = await loginUser(requestPayload);

      if (response.status === 200) {
        // Navigate based on the role submitted
        if (data.role === "Super Admin") {
          setJwt(response.body.token);
          setRole(response.body.role);
          navigate("/superadmin-panel");
        } else if (data.role === "Admin") {
          navigate("/admin-panel");
        } else {
          navigate("/worker-panel");
        }
      } else if (response.status === 0) {
        console.log(response.body);
        setError(
          "Login failed due to an internet connection issue. Please check your internet and try again."
        );
        setFailure(true);
      } else if (response.body == "User does not have the required role") {
        console.log(response.body);
        setError("You don't have the required role. Please change your role selection and try again.");
        setFailure(true);
      } else if (response.body == "Invalid password") {
        console.log(response.body);
        setError("Invalid password. Retype your passsword and try again.");
        setFailure(true);
      } else if (response.body == "User is not confirmed") {
        console.log(response.body);
        setError("Your account isn't confirmed. Please contact your admin and have them resend your confirmation email.")
        setFailure(true);
      } else if (response.body == "User is blocked") {
        console.log(response.body);
        setError("Your account has been blocked. Please contact your admin and have them unblock you.")
        setFailure(true);
      } else {
          console.log(response.body);
          setError("Login failed. Please try again.");
          setFailure(true);
      }
    } catch (err) {
      console.error(err);
      setError("An unexpected error occurred. Please try again.");
      setFailure(true);
    }
  };

  return (
    <div css={Container}>
      <div css={FormWrapper}>
        <LoginForm
          apiError={error}
          apiFailure={failure}
          submitLoginForm={submitLoginForm}
        />
      </div>
    </div>
  );
};
