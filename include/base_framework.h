/**
 * Base Framework - C/C++ Header File
 * 
 * This header file defines the framework's C ABI interfaces for external programs
 * (e.g., test.dll) to call.
 * 
 * Usage example:
 * 
 * #include "base_framework.h"
 * 
 * int main() {
 *     // Initialize framework
 *     int result = framework_init(NULL);
 *     if (result != 0) {
 *         printf("Initialization failed\n");
 *         return -1;
 *     }
 * 
 *     // Register application
 *     framework_register_app("my_app", "My Application", "1.0.0");
 * 
 *     // Start application
 *     framework_start_app("my_app");
 * 
 *     // Execute call inspection
 *     if (framework_check_call("caller_id", "interface_name", "{}") == 0) {
 *         // Inspection passed, execute operation
 *     }
 * 
 *     // Shutdown framework
 *     framework_shutdown();
 *     return 0;
 * }
 */

#ifndef BASE_FRAMEWORK_H
#define BASE_FRAMEWORK_H

#ifdef __cplusplus
extern "C" {
#endif

#ifdef _WIN32
    #ifdef BASE_FRAMEWORK_EXPORTS
        #define FRAMEWORK_API __declspec(dllexport)
    #else
        #define FRAMEWORK_API __declspec(dllimport)
    #endif
#else
    #define FRAMEWORK_API __attribute__((visibility("default")))
#endif

/**
 * Error code definitions
 */
#define FRAMEWORK_SUCCESS                    0
#define FRAMEWORK_ERROR_INIT_FAILED         -1
#define FRAMEWORK_ERROR_INVALID_PARAM       -2
#define FRAMEWORK_ERROR_MONITOR_FAILED      -3
#define FRAMEWORK_ERROR_MIDDLEWARE_FAILED   -4
#define FRAMEWORK_ERROR_OUT_OF_RESOURCE     -5
#define FRAMEWORK_ERROR_TIMEOUT             -6
#define FRAMEWORK_ERROR_NOT_IMPLEMENTED     -7
#define FRAMEWORK_ERROR_INTERNAL            -99

/**
 * Initialize framework
 * 
 * @param config_json JSON format configuration string, pass NULL for default config
 * @return 0 for success, negative for error code
 * 
 * Example:
 *   framework_init(NULL);
 *   framework_init("{\"log_level\": \"debug\"}");
 */
FRAMEWORK_API int framework_init(const char* config_json);

/**
 * Shutdown framework, release all resources
 * 
 * @return 0 for success, negative for error code
 */
FRAMEWORK_API int framework_shutdown(void);

/**
 * Register application
 * 
 * @param app_id Unique application identifier (cannot be empty)
 * @param app_name Application name
 * @param version Version number
 * @return 0 for success, negative for error code
 * 
 * Example:
 *   framework_register_app("app001", "Test Application", "1.0.0");
 */
FRAMEWORK_API int framework_register_app(
    const char* app_id,
    const char* app_name,
    const char* version
);

/**
 * Start application monitoring
 * 
 * @param app_id Application ID
 * @return 0 for success, negative for error code
 */
FRAMEWORK_API int framework_start_app(const char* app_id);

/**
 * Stop application monitoring
 * 
 * @param app_id Application ID
 * @return 0 for success, negative for error code
 */
FRAMEWORK_API int framework_stop_app(const char* app_id);

/**
 * Execute call inspection
 * 
 * Inspects calls through the middleware chain, including whitelist,
 * rate limiting, parameter validation, etc.
 * 
 * @param caller_id Caller identifier (e.g., "test.dll")
 * @param interface_name Target interface name
 * @param params_json Parameter JSON string, can be NULL
 * @return 0 for inspection passed, negative for inspection failed or error
 * 
 * Example:
 *   if (framework_check_call("test.dll", "get_data", "{}") == 0) {
 *       // Inspection passed, execute operation
 *   } else {
 *       // Inspection failed, reject operation
 *   }
 */
FRAMEWORK_API int framework_check_call(
    const char* caller_id,
    const char* interface_name,
    const char* params_json
);

/**
 * Record performance metric
 * 
 * @param metric_name Metric name
 * @param value Metric value (usually in milliseconds)
 * @return 0 for success, negative for error code
 * 
 * Example:
 *   framework_record_metric("response_time", 150.5);
 */
FRAMEWORK_API int framework_record_metric(
    const char* metric_name,
    double value
);

/**
 * Record error
 * 
 * @param error_code Error code
 * @param message Error message
 * @return 0 for success, negative for error code
 */
FRAMEWORK_API int framework_record_error(
    int error_code,
    const char* message
);

/**
 * Get framework version
 * 
 * @return Version string (statically allocated, do not free)
 */
FRAMEWORK_API const char* framework_version(void);

/**
 * Free string resource
 * 
 * @param ptr String pointer to free
 */
FRAMEWORK_API void framework_free_string(char* ptr);

/**
 * Health check
 * 
 * @return 0 for framework healthy, negative for anomaly
 */
FRAMEWORK_API int framework_health_check(void);

#ifdef __cplusplus
}
#endif

#endif /* BASE_FRAMEWORK_H */
