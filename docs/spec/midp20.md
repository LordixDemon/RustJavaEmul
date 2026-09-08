# MIDP 2.0 / JSR-135 / JSR-75 public API (coverage target)

## javax.microedition.lcdui.game
Layer, Sprite, TiledLayer, LayerManager and GameCanvas public members are registered in `java_runtime` protos. Required Sprite extras: copy constructor, `getRefPixelX/Y`, `setImage`, collision overloads, `TRANS_*`.

## javax.microedition.rms.RecordStore
open/close/add/set/get/delete/enumerate, getSize, getSizeAvailable, RecordListener, InvalidRecordIDException, RecordStoreFullException, RecordStoreNotOpenException.

## javax.microedition.io
Connector.open overloads, Connection hierarchy, HttpConnection, HttpsConnection, SocketConnection, ServerSocketConnection, UDPDatagramConnection, CommConnection, FileConnection (JSR-75).

## javax.microedition.media
Manager.createPlayer (stream + locator), playTone, Player lifecycle, VolumeControl/ToneControl/MIDIControl/StopTimeControl/MetaDataControl, MediaException.
