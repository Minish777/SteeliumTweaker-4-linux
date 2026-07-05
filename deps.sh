#!/bin/bash
# Скрипт для установки требуемых зависимостей

echo "📦 MakuTweaker 4 - Dependency Installer"
echo "======================================="

# Определяем дистрибутив
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$ID
else
    echo "❌ Не удалось определить дистрибутив Linux"
    exit 1
fi

echo "🔍 Обнаружен дистрибутив: $OS"

# Установка зависимостей в зависимости от дистрибутива
case "$OS" in
    ubuntu|debian)
        echo "📥 Установка зависимостей для Ubuntu/Debian..."
        sudo apt-get update
        sudo apt-get install -y \
            build-essential \
            libssl-dev \
            pkg-config \
            libxcb-render0-dev \
            libxcb-shape0-dev \
            libxcb-xfixes0-dev \
            libxkbcommon-dev \
            qt5-qmake \
            qt5-default
        ;;
    
    fedora)
        echo "📥 Установка зависимостей для Fedora..."
        sudo dnf groupinstall -y "Development Tools"
        sudo dnf install -y \
            openssl-devel \
            pkgconfig \
            qt5-qtbase-devel \
            libxcb-devel \
            libxkbcommon-devel
        ;;
    
    arch|manjaro)
        echo "📥 Установка зависимостей для Arch/Manjaro..."
        sudo pacman -S --needed \
            base-devel \
            openssl \
            pkg-config \
            qt5-base \
            libxcb \
            libxkbcommon
        ;;
    
    opensuse*)
        echo "📥 Установка зависимостей для openSUSE..."
        sudo zypper install -y \
            gcc \
            gcc-c++ \
            openssl-devel \
            pkg-config \
            libqt5-qtbase-devel \
            libxcb-devel \
            libxkbcommon-devel
        ;;
    
    *)
        echo "⚠️  Автоматическая установка для $OS не поддерживается"
        echo "💡 Установите вручную следующие зависимости:"
        echo "   - build-essential / gcc / clang"
        echo "   - openssl-devel"
        echo "   - pkg-config"
        echo "   - Qt5 или другой backend для графики"
        exit 1
        ;;
esac

# Проверка Rust
if ! command -v rustc &> /dev/null; then
    echo "📥 Установка Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "✅ Rust уже установлен"
fi

echo "✅ Все зависимости установлены!"
echo ""
echo "🚀 Теперь вы можете собрать проект:"
echo "   ./build.sh release"