export const create = {
  name: "create",
  description: "Make a prediction.",
  options: [
    {
      name: "condition",
      description: "The condition(s) for the prediction.",
      type: 3,
      required: true,
    },
  ],
};
