#ifndef TEST_CONFIG_H
#define TEST_CONFIG_H

#define DBG

#ifdef DBG
# define dbg(x) std::cout \
  << "LER0ever DBG::"       \
  << __FUNCTION__         \
  << "() "                \
  << "@ "                 \
  << __TIMESTAMP__        \
  << "\n"                 \
  << __FILE__             \
  << " L"                 \
  << __LINE__             \
  << " \n"                \
  << # x " = "            \
  << (x)                  \
  << std::endl            \
  << std::endl

# define dbgerr(x) std::cout \
  << "LER0ever DBG::ERROR::"   \
  << __FUNCTION__            \
  << "() "                   \
  << "@ "                    \
  << __TIMESTAMP__           \
  << "\n"                    \
  << __FILE__                \
  << " L"                    \
  << __LINE__                \
  << " \n"                   \
  << # x " = "               \
  << (x)                     \
  << std::endl               \
  << std::endl
# define everWarn(x) std::cout             \
  << "\033[33mLER0ever DBG::Warning::\033[0m" \
  << __FUNCTION__                          \
  << "() "                                 \
  << "@ "                                  \
  << __TIMESTAMP__                         \
  << "\n"                                  \
  << __FILE__                              \
  << " L"                                  \
  << __LINE__                              \
  << " \n Warning MSG:"                    \
  << (x)                                   \
  << std::endl                             \
  << std::endl
# define everErr(x) std::cout            \
  << "\033[31mLER0ever DBG::ERROR::\033[0m" \
  << __FUNCTION__                        \
  << "() "                               \
  << "@ "                                \
  << __TIMESTAMP__                       \
  << "\n"                                \
  << __FILE__                            \
  << " L"                                \
  << __LINE__                            \
  << " \n Error MSG:"                    \
  << (x)                                 \
  << std::endl                           \
  << std::endl
# define everMsg(x) std::cout          \
  << "\033[32mLER0ever DBG::Msg::\033[0m" \
  << __FUNCTION__                      \
  << "() "                             \
  << "@ "                              \
  << __TIMESTAMP__                     \
  << "\n"                              \
  << __FILE__                          \
  << " L"                              \
  << __LINE__                          \
  << " \n Notice:"                     \
  << (x)                               \
  << std::endl                         \
  << std::endl
#else // ifdef DBG
# define dbg(x)
# define dbgerr(x)
#define everMsg(x)
#define everWarn(x)
#define everErr(x)
#endif // ifdef

#endif // TEST_CONFIG_H