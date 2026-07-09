DEPENDENCIES=$(jaq --from toml --to json '.dependencies' Cargo.toml);
UPDATED=$(jaq --from toml --to toml ".dependencies=$DEPENDENCIES" crates/shared/Cargo.toml);

echo "$UPDATED" > crates/shared/Cargo.toml;
