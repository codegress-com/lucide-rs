<!--
Pull Request Template for Lucide Icons

Please fill out this template to help us review your contribution.
Remove any sections that don't apply to your changes.
-->

## 📝 Description

<!-- Provide a clear and concise description of what this PR does -->

### Type of Change

<!-- Mark the relevant option with [x] -->

- [ ] 🐛 Bug fix (non-breaking change that fixes an issue)
- [ ] ✨ New feature (non-breaking change that adds functionality)
- [ ] 💥 Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] 🔧 Refactoring (code changes that neither fix a bug nor add a feature)
- [ ] 📚 Documentation update
- [ ] 🎨 Style changes (formatting, renaming, etc.)
- [ ] 🚀 Performance improvement
- [ ] ✅ Test improvements
- [ ] 🔒 Security fix
- [ ] 📦 Dependency updates

## 🔗 Related Issues

<!-- Link any related issues or discussions -->

- Fixes #<!-- issue number -->
- Relates to #<!-- issue number -->

## 🧪 Testing

<!-- Describe how you tested your changes -->

### Manual Testing

- [ ] I have tested the changes locally
- [ ] Icons render correctly in target frameworks
- [ ] Examples build and display properly (if applicable)
- [ ] Documentation builds without errors (if applicable)

### Automated Testing

- [ ] All existing tests pass
- [ ] I have added tests for new functionality
- [ ] Tests cover edge cases and error conditions

## 📋 Checklist

<!-- Please review and check all applicable items -->

### Code Quality

- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review of my code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have removed any unnecessary debug prints or comments

### Documentation

- [ ] I have updated relevant documentation
- [ ] I have updated code comments where necessary
- [ ] Any new icons are properly categorized
- [ ] README or other docs are updated if needed

### Compatibility

- [ ] My changes don't break existing functionality
- [ ] I have tested with multiple frameworks (if applicable)
- [ ] The changes work on different platforms (if relevant)
- [ ] Backward compatibility is maintained

### Dependencies

- [ ] I have minimized the addition of new dependencies
- [ ] Any new dependencies are well-maintained and necessary
- [ ] I have updated Cargo.lock if needed

## 🔄 Breaking Changes

<!-- If this is a breaking change, describe the impact and migration path -->

### What breaks?

<!-- List any breaking changes -->

### Migration Guide

<!-- Provide guidance for users to migrate their code -->

```rust
// Before
<lucide::OldIcon class="w-6 h-6" />

// After
<lucide::NewIcon class="w-6 h-6" />
```

## 📸 Screenshots/Examples

<!-- If applicable, add screenshots or code examples -->

### Before

<!-- Show the current behavior -->

### After

<!-- Show the new behavior -->

## 📖 Additional Context

<!-- Add any other context about the problem or solution here -->

## 🎯 Focus Areas for Review

<!-- Help reviewers focus on specific areas -->

- [ ] Algorithm correctness
- [ ] Error handling
- [ ] Performance implications
- [ ] API design
- [ ] Documentation clarity
- [ ] Test coverage

---

<!--
Reviewer Guidelines:
- Check that the PR follows the contribution guidelines
- Verify tests pass and cover the changes
- Ensure documentation is updated appropriately
- Test the changes locally if possible
- Consider the impact on existing users
-->

**For Reviewers**:

- 🔍 Please test icon rendering in relevant frameworks
- 📝 Verify that documentation accurately reflects any changes
- 🧪 Check that all tests pass and new tests are appropriate
- 🎨 Verify icon categories and features are correctly assigned
- 💬 Ask questions if anything is unclear
