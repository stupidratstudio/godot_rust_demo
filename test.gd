extends Node3D

var benchmark = false
var array

func _process(delta: float) -> void:
	var v = $Player.get_points()
	var n = len(v) / 12
	$MultiMeshInstance3D.multimesh.instance_count = n
	$MultiMeshInstance3D.multimesh.buffer = v
	$MultiMeshInstance3D.rotation.x += delta * 1.1
	$MultiMeshInstance3D.rotation.y += delta * 1.2
	$MultiMeshInstance3D.rotation.z += delta * 1.3
	if benchmark:
		benchmark = false
		$Player.benchmark()
	$Label.text = "FPS:" + str(Engine.get_frames_per_second())
