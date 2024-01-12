export namespace Permission {

	export function isAllowed(permission: Permission, permissions: string[]): boolean {
		return permissions.includes(Admin) || permissions.includes(permission);
	}

	export function getDescription(permission: Permission): string {
		switch (permission) {
		}

		return "No description provided.";
	}
}

export type Permission = ;
