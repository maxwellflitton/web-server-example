/** @jsxImportSource @emotion/react */
import React from "react";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "src/ui/components/shadcnComponents/formCardContainer";
import * as styles from "./UserDetailsCard.styles";

interface UserData {
  confirmed: boolean;
  username: string;
  email: string;
  firstName: string;
  lastName: string;
  roles: string[];
  dateCreated: string;
  lastLoggedIn: string;
  blocked: boolean;
}

interface UserDetailsCardProps {
  user: UserData;
}

export function UserDetailsCard({ user }: UserDetailsCardProps) {
  const hasRole = (role: string) => user.roles.includes(role);

  return (
    <Card css={styles.cardContainer}>
      <CardHeader>
        <CardTitle className="text-2xl">User Profile</CardTitle>
      </CardHeader>
      <CardContent>
        <div css={styles.columnsContainer}>
          {/* Left column */}
          <div css={styles.column}>
            <div css={styles.infoBox}>
              <div css={styles.infoTitle}>Full Name</div>
              <div css={styles.infoContent}>
                {user.firstName} {user.lastName}
              </div>
            </div>

            <div css={styles.infoBox}>
              <div css={styles.infoTitle}>Email</div>
              <div css={styles.infoContent}>{user.email}</div>
            </div>

            <div css={styles.infoBox}>
              <div css={styles.infoTitle}>Username</div>
              <div css={styles.infoContent}>{user.username}</div>
            </div>

            <div css={styles.infoBox}>
              <div css={styles.infoTitle}>User Roles</div>
              <div css={styles.infoContent}>
                <div>Worker - {hasRole("Worker") ? "✅" : "❌"}</div>
                <div>Admin - {hasRole("Admin") ? "✅" : "❌"}</div>
                <div>Super Admin - {hasRole("Super Admin") ? "✅" : "❌"}</div>
              </div>
            </div>
          </div>

          {/* Right column */}
          <div css={styles.column}>
            <div css={styles.infoBox}>
              <div css={styles.infoTitle}>Date Created</div>
              <div css={styles.infoContent}>
                {new Date(user.dateCreated).toLocaleDateString()}
              </div>
            </div>

            <div css={styles.infoBox}>
              <div css={styles.infoTitle}>Last Login</div>
              <div css={styles.infoContent}>
                {new Date(user.lastLoggedIn).toLocaleDateString()}
              </div>
            </div>

            <div css={styles.infoBox}>
              <div css={styles.infoTitle}>Status</div>
              <div css={styles.infoContent}>
                {user.blocked ? "User Blocked ❌" : "User Active ✅"}
              </div>
            </div>

            <div css={styles.infoBox}>
              <div css={styles.infoTitle}>Email Confirmation</div>
              <div css={styles.infoContent}>
                {user.confirmed ? "Confirmed ✅" : "Not Confirmed ❌"}
              </div>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
