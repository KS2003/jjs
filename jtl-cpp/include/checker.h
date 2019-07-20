#pragma once

#include <cstdio>
#include "util.h"
#include "jtl.h"

namespace checker {

void comment(const char* format, ...) FORMAT_FN(1);

void sol_scanf(const char* format, ...) FORMAT_FN(1);

void corr_scanf(const char* format, ...) FORMAT_FN(1);

void test_scanf(const char* format, ...) FORMAT_FN(1);

void check_sol_eof();

void check_corr_eof();

void check_test_eof();

struct CheckerInput {
    /// Contestant's solution answer
    FILE* sol_answer;
    /// Correct answer (answer generated by primary solution), if requested by problem config. Otherwise, refers to /dev/null
    FILE* corr_answer;
    /// Test data
    FILE* test;
};

CheckerInput init();

/// Reads next char sequence, followed by whitespace
/// next_token() returns owning pointer to token. This pointer should be freed by free()
char* next_token(FILE* f);

enum class Outcome {

/// Checker couldn't recognize answer
    PRESENTATION_ERROR,

/// Answer was wrong
    WRONG_ANSWER,

/// Correct answer
    OK,

/// Checker is incorrect
/// for example, contestant provided more optimal answer than jury
    CHECKER_LOGIC_ERROR,
};

/// Checker exits using this function
/// If checker simply exits with e.g. exit(0) protocol will be broken and internal judging error will be diagnosed
Uninhabited finish(Outcome outcome);


/// Some comparison functions

bool compare_epsilon(long double expected, long double actual, long double epsilon);

bool compare_strings_ignore_case(const char* lhs, const char* rhs);
}