use crate::{mock::*};

//use frame_support::{assert_ok, assert_noop};

#[test]
fn test_is_queued() {
	new_test_ext().execute_with(|| {

		let player1 = 1;

		assert_eq!(MatchMaker::do_queue_size(0), 0);
		assert_eq!(MatchMaker::do_is_queued(player1), false);
		assert_eq!(MatchMaker::do_add_queue(player1, 0), true);
		assert_eq!(MatchMaker::do_is_queued(player1), true);
		MatchMaker::do_empty_queue(0);
		assert_eq!(MatchMaker::do_is_queued(player1), false);
	});
}

#[test]
fn test_try_duplicate_queue() {
	new_test_ext().execute_with(|| {

		let player1 = 1;
		let player2 = 2;

		assert_eq!(MatchMaker::do_queue_size(0), 0);
		assert_eq!(MatchMaker::do_add_queue(player1, 0), true);
		// try same bracket
		assert_eq!(MatchMaker::do_add_queue(player1, 0), false);
		// try other bracket
		assert_eq!(MatchMaker::do_add_queue(player1, 1), false);

		assert_eq!(MatchMaker::do_add_queue(player2, 1), true);
		// try same bracket
		assert_eq!(MatchMaker::do_add_queue(player2, 1), false);
		// try other bracket
		assert_eq!(MatchMaker::do_add_queue(player2, 0), false);
	});
}

#[test]
fn test_add_queue() {
	new_test_ext().execute_with(|| {

		let player1 = 1;
		let player2 = 2;

		assert_eq!(MatchMaker::do_queue_size(0), 0);
		assert_eq!(MatchMaker::do_try_match().is_empty(), true);
		assert_eq!(MatchMaker::do_add_queue(player1, 0), true);
		assert_eq!(MatchMaker::do_queue_size(0), 1);
		assert_eq!(MatchMaker::do_try_match().is_empty(), true);
		assert_eq!(MatchMaker::do_add_queue(player2,  0), true);
		assert_eq!(MatchMaker::do_queue_size(0), 2);
		assert_eq!(MatchMaker::do_try_match(), [1, 2]);
		assert_eq!(MatchMaker::do_queue_size(0), 0);
		assert_eq!(MatchMaker::do_try_match().is_empty(), true);

		assert_eq!(MatchMaker::do_add_queue(player1, 0), true);
		assert_eq!(MatchMaker::do_add_queue(player2, 0), true);
		assert_eq!(MatchMaker::do_queue_size(0), 2);
		MatchMaker::do_empty_queue(0);
		assert_eq!(MatchMaker::do_try_match().is_empty(), true);
		assert_eq!(MatchMaker::do_queue_size(0), 0);
	});
}

#[test]
fn test_brackets_count() {
	new_test_ext().execute_with(|| {
		assert_eq!(MatchMaker::brackets_count(), 3);
	});
}

#[test]
fn test_brackets() {
	new_test_ext().execute_with(|| {

		let player1 = 1; // bracket: 0
		let player2 = 2; // bracket: 0
		let player3 = 3; // bracket: 0
		let player4 = 4; // bracket: 1
		let player5 = 5; // bracket: 1
		let player6 = 6; // bracket: 2

		assert_eq!(MatchMaker::do_queue_size(0), 0);
		assert_eq!(MatchMaker::do_all_queue_size(), 0);
		assert_eq!(MatchMaker::do_add_queue(player1, 0), true);
		assert_eq!(MatchMaker::do_add_queue(player2, 0), true);
		assert_eq!(MatchMaker::do_add_queue(player3, 0), true);
		assert_eq!(MatchMaker::do_add_queue(player4, 1), true);
		assert_eq!(MatchMaker::do_add_queue(player5, 1), true);
		assert_eq!(MatchMaker::do_add_queue(player6, 2), true);
		assert_eq!(MatchMaker::do_queue_size(0), 3);
		assert_eq!(MatchMaker::do_queue_size(1), 2);
		assert_eq!(MatchMaker::do_queue_size(2), 1);
		assert_eq!(MatchMaker::do_all_queue_size(), 6);
		assert_eq!(MatchMaker::do_try_match(), [1, 2]);
		assert_eq!(MatchMaker::do_try_match(), [3, 4]);
		assert_eq!(MatchMaker::do_add_queue(player1, 0), true);
		assert_eq!(MatchMaker::do_try_match(), [1, 5]);
		assert_eq!(MatchMaker::do_try_match().is_empty(), true);
		assert_eq!(MatchMaker::do_add_queue(player5, 1), true);
		assert_eq!(MatchMaker::do_try_match(), [5, 6]);
	});
}