#!/usr/bin/env python3

import gi
import os
import os.path
import sys

gi.require_version("P2panda", "0")

from gi.repository import P2panda, Gio, GLib

def main():
    assert P2panda.Node.__gtype__.name == "P2pandaNode"
    assert P2panda.Topic.__gtype__.name == "P2pandaTopic"
    assert P2panda.Error.__gtype__.name == "P2pandaError"
    assert P2panda.MdnsDiscoveryMode.ACTIVE.__gtype__.name == "P2pandaMdnsDiscoveryMode"

    global topic_id
    topic_id = P2panda.TopicId.new()

    network_id = P2panda.NetworkId.new()
    private_key = P2panda.PrivateKey.new()
    assert private_key
    public_key = private_key.get_public_key()
    assert public_key
    node = P2panda.Node.new(private_key, None, True, network_id, None,  None, P2panda.MdnsDiscoveryMode.ACTIVE)
    assert node

    global async_tests_remaining
    async_tests_remaining = 2;

    node.spawn_async(None, node_spawn_cb, None);

    private_key2 = P2panda.PrivateKey.new()
    node2 = P2panda.Node.new(private_key2, None, True, network_id, None,  None, P2panda.MdnsDiscoveryMode.ACTIVE)
    assert node2

    node2.spawn_async(None, node2_spawn_cb, None);

    GLib.MainLoop().run()

def node_spawn_cb(node, result, user_data):
    assert user_data is None
    assert node.spawn_finish(result)

    print ("Node 1 spawned")

    global topic
    global topid_id
    topic = P2panda.Topic.new(node, topic_id, P2panda.TopicFlags.PERSISTENT | P2panda.TopicFlags.EPHEMERAL)
    assert topic
    topic.spawn_async(None, topic_spawn_cb, None)

def node2_spawn_cb(node, result, user_data):
    assert user_data is None
    assert node.spawn_finish(result)

    print ("Node 2 spawned")

    global topic2
    global topic_id
    topic2 = P2panda.Topic.new(node, topic_id, P2panda.TopicFlags.PERSISTENT | P2panda.TopicFlags.EPHEMERAL)
    assert topic2

    topic2.connect('message', message_cb, None)
    topic2.connect('ephemeral-message', message_cb, None)

    topic2.spawn_async(None, topic2_spawn_cb, None)

def message_cb(topic, public_key, datetime, bytes, user_data):
    assert user_data is None

    print("Got message")
    async_test_done()

    True

def topic2_spawn_cb(topic, result, user_data):
    assert user_data is None
    assert topic.spawn_finish(result)

    print("Topic 2 spawned")

def topic_spawn_cb(topic, result, user_data):
    assert user_data is None
    assert topic.spawn_finish(result)

    print("Topic 1 spawned")

    # Ephemeral message
    data = GLib.Bytes.new([1,2,3])
    topic.publish_async(data, True, None, publish_cb, None)

    # Persistent message
    data = GLib.Bytes.new([1,2,3])
    topic.publish_async(data, False, None, publish_cb, None)

def publish_cb(topic, result, user_data):
    assert user_data is None
    assert topic.publish_finish(result)

    print("Message published")

def async_test_done():
    global async_tests_remaining
    async_tests_remaining -= 1
    print("Global tests remaining:", async_tests_remaining)
    if async_tests_remaining == 0:
        sys.exit(0)

if __name__ == "__main__":
    main()
