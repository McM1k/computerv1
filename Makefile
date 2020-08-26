NAME = computer_v1

all: $(NAME)

$(NAME):
	cargo build --release
	cp target/release/$(NAME) .

clean:
	rm -r target

fclean: clean
	rm Cargo.lock $(NAME)

re: fclean all

norm:
	cargo clippy
	cargo fmt

test:
	cargo test