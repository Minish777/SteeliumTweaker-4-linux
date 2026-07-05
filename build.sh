#!/bin/bash
# Скрипт для сборки и установки MakuTweaker

set -e

echo "🔨 MakuTweaker 4 - Build Script"
echo "================================"

# Проверка Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust не установлен!"
    echo "💡 Установите из https://rustup.rs/"
    exit 1
fi

echo "✅ Rust найден: $(rustc --version)"

# Выбор режима сборки
if [ "$1" = "release" ]; then
    MODE="release"
    PROFILE_FLAG="--release"
    echo "📦 Собираю в режиме Release (оптимизированный)..."
else
    MODE="debug"
    PROFILE_FLAG=""
    echo "🐛 Собираю в режиме Debug (быстрая сборка)..."
fi

# Сборка
echo "🔨 Компиляция..."
cargo build $PROFILE_FLAG

# Пути
if [ "$MODE" = "release" ]; then
    BINARY="./target/release/makutweaker-4-linux"
else
    BINARY="./target/debug/makutweaker-4-linux"
fi

echo "✅ Сборка завершена!"
echo ""
echo "📍 Бинарный файл: $BINARY"
echo ""
echo "🚀 Для запуска:"
echo "   $BINARY"
echo ""
echo "🔐 Для запуска с правами администратора (для некоторых функций):"
echo "   sudo $BINARY"
echo ""

if [ "$2" = "install" ]; then
    echo "📦 Установка в /usr/local/bin..."
    sudo cp "$BINARY" /usr/local/bin/makutweaker
    sudo chmod +x /usr/local/bin/makutweaker
    echo "✅ Установлено! Запуск: makutweaker"
fi