function copyTree(tree) {
  if (tree == null) {
    return null;
  }
  return {
    value: tree.value,
    left: copyTree(tree.left),
    right: copyTree(tree.right),
  };
}

export class Zipper {
  constructor(tree, parent = null) {
    this.tree = tree;
    this.parent = parent;
  }

  static fromTree(tree) {
    return new Zipper(copyTree(tree));
  }

  toTree() {
    let zipper = this;
    while (zipper.parent != null) {
      zipper = zipper.parent;
    }
    return zipper.tree;
  }

  up() {
    return this.parent;
  }

  left() {
    if (this.tree.left == null) {
      return null;
    }
    return new Zipper(this.tree.left, this);
  }

  right() {
    if (this.tree.right == null) {
      return null;
    }
    return new Zipper(this.tree.right, this);
  }

  value() {
    return this.tree.value;
  }

  setValue(value) {
    this.tree.value = value;
    return this;
  }

  setLeft(left) {
    this.tree.left = left;
    return this;
  }

  setRight(right) {
    this.tree.right = right;
    return this;
  }
}
