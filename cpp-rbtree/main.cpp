// Compile: clang++ -std=c++17 main.cpp -o rbtree && ./rbtree

#include <cassert>
#include <functional>
#include <iostream>
#include <memory>
#include <variant>
#include <vector>

enum class Color { R, B };

struct RbTree;
struct Empty {};
struct Tree {
    Color color;
    std::shared_ptr<RbTree> left;
    int val;
    std::shared_ptr<RbTree> right;
};

struct RbTree {
    std::variant<Empty, Tree> data;
    RbTree() : data(Empty{}) {}
    explicit RbTree(Tree t) : data(std::move(t)) {}
};

static std::shared_ptr<RbTree> make_empty() {
    return std::make_shared<RbTree>();
}

static std::shared_ptr<RbTree> make_node(Color c, std::shared_ptr<RbTree> l, int v, std::shared_ptr<RbTree> r) {
    return std::make_shared<RbTree>(Tree{c, std::move(l), v, std::move(r)});
}

static const Tree* as_tree(const RbTree& node) {
    return std::get_if<Tree>(&node.data);
}

static std::shared_ptr<RbTree> balance(std::shared_ptr<RbTree> tree) {
    const Tree* t = as_tree(*tree);
    if (!t || t->color != Color::B) return tree;

    auto rebuild = [](auto a, int x, auto b, int y, auto c, int z, auto d) {
        return make_node(Color::R, make_node(Color::B, a, x, b), y, make_node(Color::B, c, z, d));
    };

    const Tree* l = as_tree(*t->left);
    const Tree* r = as_tree(*t->right);

    if (l && l->color == Color::R) {
        const Tree* ll = as_tree(*l->left);
        const Tree* lr = as_tree(*l->right);
        if (ll && ll->color == Color::R)
            return rebuild(ll->left, ll->val, ll->right, l->val, l->right, t->val, t->right);
        if (lr && lr->color == Color::R)
            return rebuild(l->left, l->val, lr->left, lr->val, lr->right, t->val, t->right);
    }
    if (r && r->color == Color::R) {
        const Tree* rl = as_tree(*r->left);
        const Tree* rr = as_tree(*r->right);
        if (rl && rl->color == Color::R)
            return rebuild(t->left, t->val, rl->left, rl->val, rl->right, r->val, r->right);
        if (rr && rr->color == Color::R)
            return rebuild(t->left, t->val, r->left, r->val, rr->left, rr->val, rr->right);
    }

    return tree;
}

static std::shared_ptr<RbTree> insert(std::shared_ptr<RbTree> tree, int val) {
    std::function<std::shared_ptr<RbTree>(std::shared_ptr<RbTree>)> ins =
        [&ins, val](std::shared_ptr<RbTree> t) -> std::shared_ptr<RbTree> {
            const Tree* node = as_tree(*t);
            if (!node)
                return make_node(Color::R, make_empty(), val, make_empty());
            if (val < node->val)
                return balance(make_node(node->color, ins(node->left), node->val, node->right));
            if (val > node->val)
                return balance(make_node(node->color, node->left, node->val, ins(node->right)));
            return t;
        };

    auto result = ins(tree);
    const Tree* node = as_tree(*result);
    if (node && node->color == Color::R)
        return make_node(Color::B, node->left, node->val, node->right);
    return result;
}

static bool contains(const std::shared_ptr<RbTree>& tree, int val) {
    const Tree* node = as_tree(*tree);
    if (!node) return false;
    if (val < node->val) return contains(node->left, val);
    if (val > node->val) return contains(node->right, val);
    return true;
}

static std::vector<int> to_sorted_list(const std::shared_ptr<RbTree>& tree) {
    const Tree* node = as_tree(*tree);
    if (!node) return {};
    auto result = to_sorted_list(node->left);
    result.push_back(node->val);
    auto right = to_sorted_list(node->right);
    result.insert(result.end(), right.begin(), right.end());
    return result;
}

int main() {
    auto tree = make_empty();

    std::vector<int> values = {7, 3, 18, 10, 22, 8, 11, 26, 2, 6, 13};
    std::cout << "Inserting: ";
    for (int v : values) { std::cout << v << " "; tree = insert(tree, v); }
    std::cout << "\n";

    std::cout << "Sorted: ";
    for (int v : to_sorted_list(tree)) std::cout << v << " ";
    std::cout << "\n";

    for (int v : {10, 5, 22})
        std::cout << "contains(" << v << ") = " << contains(tree, v) << "\n";

    auto t = make_empty();
    for (int i = 1; i <= 20; i++) t = insert(t, i);
    auto sorted = to_sorted_list(t);
    for (int i = 0; i < 20; i++) assert(sorted[i] == i + 1);
    std::cout << "Ascending 1..20: OK\n";

    t = make_empty();
    for (int i = 20; i >= 1; i--) t = insert(t, i);
    sorted = to_sorted_list(t);
    for (int i = 0; i < 20; i++) assert(sorted[i] == i + 1);
    std::cout << "Descending 20..1: OK\n";

    t = make_empty();
    for (int v : {5, 5, 5, 3, 3}) t = insert(t, v);
    assert((to_sorted_list(t) == std::vector<int>{3, 5}));
    std::cout << "Duplicates ignored: OK\n";

    std::cout << "\nAll tests passed!\n";
    return 0;
}
