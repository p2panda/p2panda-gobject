#pragma once

#include <glib-object.h>
#include <gio/gio.h>
#include <stdint.h>

G_BEGIN_DECLS

/**
 * P2pandaNode::system-event:
 *
 * Emitted on system events
 */

/**
 * P2pandaTopic::message:
 * @topic:
 * @public_key: The public key of the origin node of the message
 * @datetime: The timestamp
 * @bytes: The message
 *
 * Emitted on incomming persistent message
 */

/**
 * P2pandaTopic::ephemeral-message:
 * @topic:
 * @public_key: The public key of the origin node of the message
 * @datetime: The timestamp
 * @bytes: The content of the message
 *
 * Emitted on incomming ephemeral message
 */

/**
 * P2pandaTopic::sync-started:
 * @topic:
 * @remote_node_id: The public key for the remote node
 * @session_id:
 * @incomming_operations:
 * @outgoing_operations:
 * @incoming_bytes:
 * @outgoing_bytes:
 *
 * Emitted for errors
 */

/**
 * P2pandaTopic::sync-ended:
 * @topic:
 * @remote_node_id: The public key for the remote node
 * @session_id:
 *
 * Emitted when a topic finishes syncing
 */

/**
 * P2pandaTopic::error:
 * @topic:
 * @error:
 *
 * Emitted when a topic finishes syncing
 */

/**
 * P2pandaNode:
 *
 * [class@Node] main entry point.
 *
 */
#define P2PANDA_TYPE_NODE (p2panda_node_get_type())
G_DECLARE_FINAL_TYPE(P2pandaNode, p2panda_node, P2PANDA, NODE, GObject)

/**
 * P2pandaTopic:
 *
 * The subscription handle to a topic.
 */
#define P2PANDA_TYPE_TOPIC (p2panda_topic_get_type())
G_DECLARE_FINAL_TYPE(P2pandaTopic, p2panda_topic, P2PANDA, TOPIC, GObject)

/**
 * P2pandaMdnsDiscoveryMode:
 * @P2PANDA_MDNS_DISCOVERY_MODE_ACTIVE: Broadcast this node and look for other nodes via MDNS
 * @P2PANDA_MDNS_DISCOVERY_MODE_PASSIVE: Look for other nodes vie MDNS but do not broadcast this node
 *
 */
typedef enum
{
    P2PANDA_MDNS_DISCOVERY_MODE_ACTIVE,
    P2PANDA_MDNS_DISCOVERY_MODE_PASSIVE,
} P2pandaMdnsDiscoveryMode;

GType p2panda_mdns_discovery_mode_get_type(void);

typedef struct _P2pandaNetworkId P2pandaNetworkId;
GType p2panda_network_id_get_type(void);
#define P2PANDA_TYPE_NETWORK_ID (p2panda_network_id_get_type())

/**
 * p2panda_network_id_new:
 *
 * return:
 */
P2pandaNetworkId *p2panda_network_id_new(void);

P2pandaNetworkId *p2panda_network_id_copy(P2pandaNetworkId *network_id);
void p2panda_network_id_free(P2pandaNetworkId *network_id);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(P2pandaNetworkId, p2panda_network_id_free)

/**
 * p2panda_network_id_new_from_data:
 * @data: (array fixed-size=32):
 *
 * Return: a new [struct@NetworkId]
 */
P2pandaNetworkId *p2panda_network_id_new_from_data(const uint8_t data[32]);

/**
 * p2panda_network_id_get_data:
 * @network_id:
 *
 * Retrieves the raw bytes of the [struct@NetworkId]
 *
 * Returns: (transfer none) (array fixed-size=32):
 */
const uint8_t* p2panda_network_id_get_data(P2pandaNetworkId *network_id);

typedef struct _P2pandaNodeId P2pandaNodeId;
GType p2panda_node_id_get_type(void);
#define P2PANDA_TYPE_NODE_ID (p2panda_node_id_get_type())

P2pandaNodeId *p2panda_node_id_copy(P2pandaNodeId *node_id);
void p2panda_node_id_free(P2pandaNodeId *node_id);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(P2pandaNodeId, p2panda_node_id_free)

/**
 * p2panda_node_id_new_from_data:
 * @data: (array fixed-size=32):
 * @relay_url: (nullable): The url of a iroh Relay Server
 * @error:
 *
 * Returns: (nullable): a new [struct@NodeId]
 */
P2pandaNodeId *p2panda_node_id_new_from_data(const uint8_t data[32],
                                             GUri *relay_url,
                                             GError **error);

/**
 * p2panda_node_id_get_data:
 * @node_id:
 *
 * Retrieves the raw bytes of the [struct@NodeId]
 *
 * Returns: (transfer none) (array fixed-size=32):
 */
const uint8_t* p2panda_node_id_get_data(P2pandaNodeId *node_id);

/**
 * P2pandaTopicFlags:
 * @P2PANDA_TOPIC_FLAGS_NONE:
 * @P2PANDA_TOPIC_FLAGS_PERSISTENT:
 * @P2PANDA_TOPIC_FLAGS_EPHEMERAL:
 * @P2PANDA_TOPIC_FLAGS_FROM_START:
 *
 */
typedef enum
{
    P2PANDA_TOPIC_FLAGS_NONE,
    P2PANDA_TOPIC_FLAGS_PERSISTENT,
    P2PANDA_TOPIC_FLAGS_EPHEMERAL,
    P2PANDA_TOPIC_FLAGS_FROM_START,
} P2pandaTopicFlags;

GType p2panda_topic_flags_get_type(void);

typedef struct _P2pandaTopicId P2pandaTopicId;
GType p2panda_topic_id_get_type(void);
#define P2PANDA_TYPE_TOPIC_ID (p2panda_topic_id_get_type())

/**
 * p2panda_topic_id_new:
 *
 * return:
 */
P2pandaTopicId *p2panda_topic_id_new(void);

P2pandaTopicId *p2panda_topic_id_copy(P2pandaTopicId *topic_id);
void p2panda_topic_id_free(P2pandaTopicId *topic_id);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(P2pandaTopicId, p2panda_topic_id_free)

/**
 * p2panda_topic_id_new_from_data:
 * @data: (array fixed-size=32):
 *
 * Return: a new [struct@TopicId]
 */
P2pandaTopicId *p2panda_topic_id_new_from_data(const uint8_t data[32]);

/**
 * p2panda_topic_id_get_data:
 * @topic_id:
 *
 * Retrieves the raw bytes of the [struct@TopicId]
 *
 * Returns: (transfer none) (array fixed-size=32):
 */
const uint8_t* p2panda_topic_id_get_data(P2pandaTopicId *topic_id);

typedef struct _P2pandaPrivateKey P2pandaPrivateKey;
GType p2panda_private_key_get_type(void);
#define P2PANDA_TYPE_PRIVATE_KEY (p2panda_private_key_get_type())

P2pandaPrivateKey *p2panda_private_key_copy(P2pandaPrivateKey *private_key);
void p2panda_private_key_free(P2pandaPrivateKey *private_key);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(P2pandaPrivateKey, p2panda_private_key_free)

/**
 * p2panda_private_key_new:
 *
 * return:
 */
P2pandaPrivateKey *p2panda_private_key_new(void);

/**
 * p2panda_private_key_new_from_data:
 * @data: (array fixed-size=32):
 *
 * return:
 */
P2pandaPrivateKey *p2panda_private_key_new_from_data(const uint8_t data[32]);

/**
 * p2panda_private_key_get_data:
 * @private_key:
 *
 * Retrieves the raw bytes of the [struct@PrivateKey]
 *
 * Returns: (transfer none) (array fixed-size=32):
 */
const uint8_t* p2panda_private_key_get_data(P2pandaPrivateKey *private_key);

typedef struct _P2pandaPublicKey P2pandaPublicKey;
GType p2panda_public_key_get_type(void);
#define P2PANDA_TYPE_PUBLIC_KEY (p2panda_public_key_get_type())

P2pandaPublicKey *p2panda_public_key_copy(P2pandaPublicKey *public_key);
void p2panda_public_key_free(P2pandaPublicKey *public_key);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(P2pandaPublicKey, p2panda_public_key_free)

/**
 * p2panda_private_key_get_public_key:
 * @private_key:
 *
 * Retrieves the the [struct@PublicKey]
 *
 * Returns: (transfer full):
 */
P2pandaPublicKey *p2panda_private_key_get_public_key(P2pandaPrivateKey *private_key);

/**
 * p2panda_public_key_new_from_data:
 * @data: (array fixed-size=32):
 * @error:
 *
 * Returns: (nullable): a new [struct@PublicKey]
 */
P2pandaPublicKey *p2panda_public_key_new_from_data(const uint8_t data[32],
                                                   GError **error);

/**
 * p2panda_public_key_get_data:
 * @public_key:
 *
 * Retrieves the raw bytes of the [struct@PublicKey]
 *
 * Returns: (transfer none) (array fixed-size=32):
 */
const uint8_t* p2panda_public_key_get_data(P2pandaPublicKey *public_key);

typedef struct _P2pandaHash P2pandaHash;
GType p2panda_hash_get_type(void);
#define P2PANDA_TYPE_HASH (p2panda_hash_get_type())

P2pandaHash *p2panda_hash_copy(P2pandaHash *hash);
void p2panda_hash_free(P2pandaHash *hash);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(P2pandaHash, p2panda_hash_free)

/*
typedef struct _P2pandaRelayUrl P2pandaRelayUrl;
GType p2panda_relay_url_get_type(void);
#define P2PANDA_TYPE_RELAY_URL (p2panda_relay_url_get_type())

P2pandaRelayUrl *p2panda_relay_url_copy(P2pandeRelayUrl *relay_url);
void p2panda_relay_url_free(P2pandaRelayUrl *relay_url);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(P2pandaRelayUrl, p2panda_relay_url_free)
*/

/**************** P2pandaNode ****************/

/**
 * p2panda_node_new:
 * @private_key: (nullable): The private key to be used for this node
 * @database_url: (nullable): A sqlite address
 * @network_id: (nullable): The network id of the network to join
 * @relay_url: (nullable): The url of a iroh Relay Server
 * @bootstrap_node_id: (nullable): The address of the bootstrap node
 * @mdns_mode: The MDNS decovery mode 
 *
 * Creates a new node with the given options.
 *
 * Returns: (transfer full): a new [class@Node]
 */
P2pandaNode *p2panda_node_new(P2pandaPrivateKey *private_key,
			      const char *database_url,
			      P2pandaNetworkId *network_id,
			      GUri *relay_url,
			      P2pandaNodeId *bootstrap_node_id,
			      P2pandaMdnsDiscoveryMode mdns_mode);

/**
 * p2panda_node_spawn_async:
 * @node:
 * @cancellable: (nullable):
 * @callback:
 * @user_data: user data to pass to @callback
 *
 */
void p2panda_node_spawn_async(P2pandaNode *node,
			      GCancellable *cancellable,
    			      GAsyncReadyCallback callback,
    			      gpointer user_data);

/**
 * p2panda_node_spawn_finish:
 * @node:
 * @result: A [iface@Gio.AsyncResult]
 * @error:
 *
 * Finishes the [method@Node.spawn_async] call.
 *
 * Returns:
 */
gboolean p2panda_node_spawn_finish(P2pandaNode *node,
				   GAsyncResult *result,
    				   GError **error);

/**************** P2pandaTopic  ****************/

/**
 * p2panda_topic_new:
 * @node:
 * @topic_id: (nullable):
 * @flags:
 *
 * Create a topic handle for the give [class@Node]
 *
 * Returns: (transfer full): a new [class@Node]
 */
P2pandaTopic *p2panda_topic_new(P2pandaNode *node,
				P2pandaTopicId *topic_id,
				uint flags);

/**
 * p2panda_topic_spawn_async:
 * @topic:
 * @cancellable: (nullable):
 * @callback:
 * @user_data: user data to pass to @callback
 *
 */
void p2panda_topic_spawn_async(P2pandaTopic *topic,
			       GCancellable *cancellable,
    			       GAsyncReadyCallback callback,
    			       gpointer user_data);

/**
 * p2panda_topic_spawn_finish:
 * @topic:
 * @result: A [iface@Gio.AsyncResult]
 * @error:
 *
 * Finishes the [method@Topic.spawn_async] call.
 *
 * Returns:
 */
gboolean p2panda_topic_spawn_finish(P2pandaTopic *topic,
				    GAsyncResult *result,
    				    GError **error);

/**
 * p2panda_topic_publish_async:
 * @topic:
 * @bytes: (transfer full):
 * @ephemeral: Whether this message should be ephemeral or persistent
 * @cancellable: (nullable):
 * @callback:
 * @user_data: user data to pass to @callback
 *
 */
void p2panda_topic_publish_async(P2pandaTopic *topic,
			         GBytes *bytes,
			         gboolean ephemeral,
			         GCancellable *cancellable,
    			         GAsyncReadyCallback callback,
    			         gpointer user_data);

/**
 * p2panda_publish_publish_finish:
 * @topic:
 * @result: A [iface@Gio.AsyncResult]
 * @error:
 *
 * Finishes the [method@Topic.publish_async] call.
 *
 * Returns:
 */
gboolean p2panda_topic_publish_finish(P2pandaTopic *topic,
				      GAsyncResult *result,
    				      GError **error);

/**************** P2pandaError ****************/

/**
 * P2pandaError:
 * @P2PANDA_LOADER_ERROR_FAILED:
 * @P2PANDA_LOADER_ERROR_UNKNOWN_IMAGE_FORMAT:
 * @P2PANDA_LOADER_ERROR_NO_MORE_FRAMES:
 *
 * Errors that can appear while loading images.
 *
 * Since: 2.0
 */
typedef enum
{
    /**
     * P2PANDA_ERROR_FAILED:
     *
     * Generic type for all other errors.
     */
    P2PANDA_ERROR_FAILED,
    /**
     * P2PANDA_ERROR_SPAWN_NODE:
     *
     * Failed to spawn node.
     */
    P2PANDA_ERROR_SPAWN_NODE,
    /**
     * P2PANDA_ERROR_SPAWN_TOPIC:
     *
     * Failed to spawn topic.
     */
    P2PANDA_ERROR_SPAWN_TOPIC,
    /**
     * P2PANDA_ERROR_NOT_SPAWNED:
     *
     * Topic or Node wasn't spawned.
     */
    P2PANDA_ERROR_NOT_SPAWNED,

    /**
     * P2PANDA_ERROR_DECODING:
     *
     * Unable to decode a message.
     */
    P2PANDA_ERROR_DECODING,

    /**
     * P2PANDA_ERROR_REPLAY:
     *
     * Unable to replay a message.
     */
    P2PANDA_ERROR_REPLAY,

    /**
     * P2PANDA_ERROR_HAS_NO_PERSISTENT:
     *
     * Message was send to a topic without the PERSISTENT flag.
     */
    P2PANDA_ERROR_HAS_NO_PERSISTENT,

    /**
     * P2PANDA_ERROR_HAS_NO_EPHEMERAL:
     *
     * Message was send to a topic without the EPHEMERAL flag.
     */
    P2PANDA_ERROR_HAS_NO_EPHEMERAL,

    /**
     * P2PANDA_ERROR_PUBLISH:
     *
     * Failed to publish a message.
     */
    P2PANDA_ERROR_PUBLISH,

    /**
     * P2PANDA_ERROR_SIGNATURE:
     *
     * Invalid sigature, when parsing [struct@NodeId] or [struct@PrivateKey].
     */
    P2PANDA_ERROR_SIGNATURE,

} P2pandaError;

/**
 * p2panda_error_quark:
 *
 * Error quark for [error@Error]
 *
 * Returns: The error domain
 */
GQuark p2panda_error_quark(void) G_GNUC_CONST;

#define P2PANDA_ERROR (p2panda_error_quark())
GType p2panda_error_get_type(void);

G_END_DECLS
