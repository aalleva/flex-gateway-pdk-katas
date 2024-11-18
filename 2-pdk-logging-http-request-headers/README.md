# Kata 2: Logging HTTP Request Headers

## Objective

Extend the basic logging policy to log each HTTP request header received. This exercise builds on Kata 1 by implementing header inspection and logging. This will help you practice accessing and iterating over HTTP headers in an Anypoint Flex Gateway PDK policy.

### Requirements

1. Modify the existing project from Kata 1 using the Anypoint Flex Gateway PDK.
2. Implement a policy that:
   - Logs each incoming HTTP request header.
   - Logs both the header name and the header value for each incoming request.
   - Allows the request to continue without any changes.

### Steps

1. **Project Setup**:
   - Use the project created in Kata 1.
   - Ensure the project is still configured to be deployable on the Flex Gateway.

2. **Logging HTTP Headers**:
   - Inside the policy, access all incoming headers.
   - Iterate over each header and log its name and value.

3. **Compile the Project**:
   - Compile the project using the Anypoint Flex Gateway PDK to produce the updated policy.

### Expected Output

When this policy is running, you should see each header of the HTTP request logged individually, showing both header names and values in the system where it's deployed (e.g., Flex Gateway).