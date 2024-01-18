use crate::{mock::*, ClubList, Clubs};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::DispatchError;

const ALICE: u64 = 1;
const BOB: u64 = 2;

#[test]
fn assign_club_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(ClubsPallet::assign_club(Origin::root(), ALICE, Clubs::BadGuys));

		assert_eq!(ClubList::<Test>::get(ALICE), Some(Clubs::BadGuys));
		assert_eq!(ClubList::<Test>::get(BOB), None);

		// Check if assigning club "Empty" removes the value from storage
		assert_ok!(ClubsPallet::assign_club(Origin::root(), ALICE, Clubs::Empty));
		assert_eq!(ClubList::<Test>::get(ALICE), None);

		// Check if removing twice doesn't throw an error
		assert_ok!(ClubsPallet::assign_club(Origin::root(), ALICE, Clubs::Empty));
		assert_eq!(ClubList::<Test>::get(ALICE), None);
	});
}

#[test]
fn assign_club_should_fail() {
	new_test_ext().execute_with(|| {
		// Case 0: Signed by non-root origin
		assert_noop!(
			ClubsPallet::assign_club(Origin::signed(ALICE), BOB, Clubs::CoolGuys),
			DispatchError::BadOrigin
		);

		// Case 1: Unsigned origin
		assert_noop!(
			ClubsPallet::assign_club(Origin::none(), ALICE, Clubs::EvenCoolerGuys),
			DispatchError::BadOrigin
		);

		assert_eq!((ClubList::<Test>::get(ALICE), ClubList::<Test>::get(BOB)), (None, None));
	})
}
