export namespace Role {
	// An application-wide admin; can do anything, in any account.
	export const Admin = "ROLE_ADMIN";
	
	// A regular user of the application.
	export const User = "ROLE_USER";
	
	// An automated service user; like an Admin, can do anything in any account.
	export const Service = "ROLE_SERVICE";
	
}

export type Permission = typeof Role.Admin | typeof Role.User | typeof Role.Service;
