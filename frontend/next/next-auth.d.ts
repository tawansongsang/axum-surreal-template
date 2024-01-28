import NextAuth, { DefaultSession } from "next-auth"

declare module "next-auth" {
  interface User {
    role: "USER" | "ADMIN",
    auth_token: string,
  }
}

export type ExtendedUser = DefaultSession["user"] & {
  role: "ADMIN" | "USER",
  auth_token: string,
}

declare module "next-auth" {
  interface Session {
    user: ExtendedUser;
  }
}

// declare module "next-auth/jwt" {
//   interface JWT {
//     role?: "USER" | "ADMIN"
//   }
// }
