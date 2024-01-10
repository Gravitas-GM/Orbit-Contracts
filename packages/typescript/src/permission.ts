export namespace Permission {
	// Allows a user to do anything. This is a dangerous permission to grant.
	export const Admin = "admin";
	
	// Allows a user to manage quiz-related tasks, such as questions and tags.
	export const ManageQuiz = "manage quiz";
	
	// Allows a user to manage other users.
	export const ManageUser = "manage user";
	
	// Allows a user to manage account settings.
	export const ManageAccount = "manage account";
	

	export function isAllowed(permission: Permission, permissions: string[]): boolean {
		return permissions.includes(Admin) || permissions.includes(permission);
	}

	export function getDescription(permission: Permission): string {
		switch (permission) {
			case Permission.Admin:
				return "Allows a user to do anything. This is a dangerous permission to grant.";
			case Permission.ManageQuiz:
				return "Allows a user to manage quiz-related tasks, such as questions and tags.";
			case Permission.ManageUser:
				return "Allows a user to manage other users.";
			case Permission.ManageAccount:
				return "Allows a user to manage account settings.";
		}

		return "No description provided.";
	}
}

export type Permission = typeof Permission.Admin | typeof Permission.ManageQuiz | typeof Permission.ManageUser | typeof Permission.ManageAccount;
