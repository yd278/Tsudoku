#include <gtest/gtest.h>
#include <solvers/DLX.h>


TEST(DLXExceptionTest,MultipleSolutionDetection){
    try {
    solve("010000000300900020005000007020040003038020150400050060200000900009008002000000030");
    FAIL() << "Cannot catch multiple solution";
    } catch (std::invalid_argument const &e) {
        EXPECT_STREQ(e.what(), "multiple solutions");
    } catch(...){
        FAIL() << "Caught unexpected exception type";
    }
}
TEST(DLXExceptionTest,MultipleSolutionDetection2){
    try {
    solve("000002000000080000050000000000300800000900400000000000000806000000000070000000008");
    FAIL() << "Cannot catch multiple solution";
    } catch (std::invalid_argument const &e) {
        EXPECT_EQ(std::strlen(e.what()),81);
    } catch(...){
        FAIL() << "Caught unexpected exception type";
    }
}


TEST(DLXExceptionTest,NoSolutionDetection){
    try {
    solve("010000000300960020005000017020040003038020150400050060200000900049078002000000030");
    FAIL() << "Cannot catch no solution";
    } catch (std::invalid_argument const &e) {
        EXPECT_EQ(std::strlen(e.what()),81);
    } catch(...){
        FAIL() << "Caught unexpected exception type";
    }
}
