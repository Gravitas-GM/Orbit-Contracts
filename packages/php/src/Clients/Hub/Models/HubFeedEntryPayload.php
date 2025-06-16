<?php
	namespace Gravitas\Orbit\Contracts\Clients\Hub\Models;

	class HubFeedEntryPayload {
		public string $content;

		/**
		 * @var HubFeedEntryRecipientPayload[]
		 */
		public array $recipients;

	}
