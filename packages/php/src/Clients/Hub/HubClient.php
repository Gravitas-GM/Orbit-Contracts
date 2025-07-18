<?php
	namespace Gravitas\Orbit\Contracts\Clients\Hub;

	use Gravitas\Orbit\Contracts\Clients\AbstractClient;
	use Gravitas\Orbit\Contracts\Clients\Hub\Models\HubAccount;
	use Gravitas\Orbit\Contracts\Clients\Hub\Models\HubFeedEntry;
	use Gravitas\Orbit\Contracts\Clients\Hub\Models\HubFeedEntryPayload;
	use Gravitas\Orbit\Contracts\Clients\Hub\Models\HubUser;
	use Gravitas\Orbit\Contracts\Clients\ProjectionInterface;
	use Gravitas\Orbit\Contracts\Clients\QueryInterface;
	use Symfony\Component\Serializer\SerializerInterface;
	use Symfony\Contracts\HttpClient\Exception\ExceptionInterface;
	use Symfony\Contracts\HttpClient\HttpClientInterface;

	class HubClient extends AbstractClient {
		public function __construct(
			HttpClientInterface $orbitHubClient,
			SerializerInterface $serializer,
			string $responseFormat = 'json',
		) {
			parent::__construct($orbitHubClient, $serializer, $responseFormat);
		}

		/**
		 * @param ProjectionInterface|null $projection
		 * @param QueryInterface|null      $query
		 *
		 * @return HubUser[]
		 * @throws ExceptionInterface {@see static::deserializeArray()}
		 */
		public function users(ProjectionInterface $projection = null, QueryInterface $query = null): array {
			$response = $this->client->request(static::METHOD_GET, '/users', $this->buildOptions($projection, $query));
			return $this->deserializeArray(HubUser::class, $response);
		}

		/**
		 * @param int                      $id
		 * @param ProjectionInterface|null $projection
		 *
		 * @return HubUser|null
		 * @throws ExceptionInterface {@see static::deserialize()}
		 */
		public function user(int $id, ProjectionInterface $projection = null): ?HubUser {
			$response = $this->client->request(static::METHOD_GET, '/users/' . $id, $this->buildOptions($projection));

			if ($response->getStatusCode() === 404)
				return null;

			return $this->deserialize(HubUser::class, $response);
		}

		/**
		 * @param ProjectionInterface|null $projection
		 * @param QueryInterface|null      $query
		 *
		 * @return HubAccount[]
		 * @throws ExceptionInterface {@see static::deserializeArray()}
		 */
		public function accounts(ProjectionInterface $projection = null, QueryInterface $query = null): array {
			$response = $this->client->request(
				static::METHOD_GET,
				'/accounts',
				$this->buildOptions($projection, $query),
			);

			return $this->deserializeArray(HubAccount::class, $response);
		}

		/**
		 * @param int                      $id
		 * @param ProjectionInterface|null $projection
		 *
		 * @return HubAccount|null
		 * @throws ExceptionInterface {@see static::deserialize()}
		 */
		public function account(int $id, ProjectionInterface $projection = null): ?HubAccount {
			$response = $this->client->request(
				static::METHOD_GET,
				'/accounts/' . $id,
				$this->buildOptions($projection),
			);

			if ($response->getStatusCode() === 404)
				return null;

			return $this->deserialize(HubAccount::class, $response);
		}

		/**
		 * @param HubFeedEntryPayload $payload
		 *
		 * @return HubFeedEntry
		 * @throws ExceptionInterface {@see static::deserialize()}
		 */
		public function createFeedEntry(HubFeedEntryPayload $payload): HubFeedEntry {
			$response = $this->client->request(
				static::METHOD_PUT,
				'/feed',
				$this->buildOptions(payload: $payload),
			);

			return $this->deserialize(HubFeedEntry::class, $response);
		}
	}
