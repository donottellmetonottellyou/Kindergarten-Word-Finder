extends ExtWordManager

func _on_valid_word_created(word):
	var show_word = load("res://scenes/show_word.tscn").instantiate()
	show_word.add_child(word)
	get_tree().root.add_child(show_word)
	queue_free()
