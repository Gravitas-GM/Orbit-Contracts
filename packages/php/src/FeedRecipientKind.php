<?php
	namespace Gravitas\Orbit\Contracts;

	final class FeedRecipientKind {
		// A single, regular user
		public const User = "user";

		// An entire account
		public const Account = "account";

		// All current members of a department
		public const Department = "department";

		private function __construct() {}
	}
