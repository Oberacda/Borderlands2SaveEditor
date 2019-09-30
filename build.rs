extern crate protoc_rust;
use std::fs;

use protoc_rust::Customize;

fn main() {
	protoc_rust::run(protoc_rust::Args {
	    out_dir: "src/protos",
	    input: &[
                "./protos/BankSlot.proto",
				"./protos/ChallengeData.proto",
				"./protos/ChosenVehicleCustomization.proto",
				"./protos/Color.proto",
				"./protos/DLCExpansionData.proto",
				"./protos/GoldenKeys.proto",
				"./protos/GUID.proto",
				"./protos/InventorySlotData.proto",
				"./protos/ItemData.proto",
				"./protos/ItemMemento.proto",
				"./protos/LockoutData.proto",
				"./protos/MissionData.proto",
				"./protos/MissionPlaythroughData.proto",
				"./protos/MissionStatus.proto",
				"./protos/OneOffLevelChallengeData.proto",
				"./protos/PackedItemDataOptional.proto",
				"./protos/PackedItemData.proto",
				"./protos/PackedWeaponDataOptional.proto",
				"./protos/PackedWeaponData.proto",
				"./protos/PendingMissionRewards.proto",
				"./protos/PlayerMark.proto",
				"./protos/QuickWeaponSlot.proto",
				"./protos/RegionGameStageData.proto",
				"./protos/ResourceData.proto",
				"./protos/SkillData.proto",
				"./protos/UIPreferencesData.proto",
				"./protos/WeaponData.proto",
				"./protos/WeaponMemento.proto",
				"./protos/WillowTwoPlayerSaveGame.proto",
                "./protos/WorldDiscoveryData.proto"
            ],
	    includes: &["protos"],
	    customize: Customize {
	      ..Default::default()
	    },
	}).expect("protoc");
}
