_default:
    @just --list

# Runs clippy on the sources
check:
    cargo clippy -- -D warnings

# Runs unit tests
test:
    cargo test

# Create virtualenv if not exists and install dependencies
venv venv_dir='./venv':
    if {{path_exists(venv_dir)}}; \
        then virtualenv venv -ppython3.9 && venv/bin/pip install seaborn; \
    fi

set positional-arguments

# Makes kernel plot
plot temp_dir='/tmp/plot_tmp': venv
    mkdir -p {{temp_dir}}
    cargo r --quiet --example kernel_data -- {{temp_dir}}
    venv/bin/python docs/plot_kernels.py {{temp_dir}}
    cp {{temp_dir}}/kernel_shapes.png docs/
    rm -rf {{temp_dir}}
