#include <cstdio>
#include <cmath>
#include <cstring>
#include <ctime>
#include <iostream>
#include <algorithm>
#include <set>
#include <vector>
#include <sstream>
#include <typeinfo>
#include <fstream>

#define REP(i, s, n) for (int i=(int)(s); i < (int)(n); ++i)

using namespace std;

class TreesAndBrackets {
public:
  vector<string> ch(const string &t) {
    vector<string> ret;
    int pos = 1;
    int last = 1;
    REP(i, 1, t.length() - 1) {
      if (t[i] == '(') {
	pos += 1;
      } else {
	pos -= 1;
      }
      if (pos == 1) {
	ret.push_back(t.substr(last, i + 1 - last));
	last = i + 1;
      }
    }
    return ret;
    vector<pair<int, string> > ret2;
    int cnt = 0;
    REP(i, 0, ret.size()) {
      if (ret[i] == "()") {
	cnt += 1;
      } else {
	ret2.push_back(make_pair(cnt, ret[i]));
	cnt = 0;
      }
    }
    ret2.push_back(make_pair(cnt, "defunct"));
    if (0) {
      cerr << "Tree " << t << ":" << endl;
      REP(i, 0, ret2.size()) {
	cerr << ret2[i].first << " " << ret2[i].second << endl;
      }
    }
  }
  bool ok(const string &t1, const string &t2) {
    vector<string> ch1 = ch(t1);
    vector<string> ch2 = ch(t2);
    if (ch1.size() < ch2.size()) return false;
    int sz1 = ch1.size();
    int sz2 = ch2.size();
    int i = 0;
    REP(pos, 0, sz1) {
      if (i >= sz2) break;
      if (ok(ch1[pos], ch2[i])) {
	i++;
      }
    }
    return i>=sz2;
  }
  string check(string t1, string t2) {
    return ok(t1, t2)?"Possible":"Impossible";
  }
};

// CUT begin
ifstream data("TreesAndBrackets.sample");

string next_line() {
    string s;
    getline(data, s);
    return s;
}

template <typename T> void from_stream(T &t) {
    stringstream ss(next_line());
    ss >> t;
}

void from_stream(string &s) {
    s = next_line();
}

template <typename T>
string to_string(T t) {
    stringstream s;
    s << t;
    return s.str();
}

string to_string(string t) {
    return "\"" + t + "\"";
}

bool do_test(string t1, string t2, string __expected) {
    time_t startClock = clock();
    TreesAndBrackets *instance = new TreesAndBrackets();
    string __result = instance->check(t1, t2);
    double elapsed = (double)(clock() - startClock) / CLOCKS_PER_SEC;
    delete instance;

    if (__result == __expected) {
        cout << "PASSED!" << " (" << elapsed << " seconds)" << endl;
        return true;
    }
    else {
        cout << "FAILED!" << " (" << elapsed << " seconds)" << endl;
        cout << "           Expected: " << to_string(__expected) << endl;
        cout << "           Received: " << to_string(__result) << endl;
        return false;
    }
}

int run_test(bool mainProcess, const set<int> &case_set, const string command) {
    int cases = 0, passed = 0;
    while (true) {
        if (next_line().find("--") != 0)
            break;
        string t1;
        from_stream(t1);
        string t2;
        from_stream(t2);
        next_line();
        string __answer;
        from_stream(__answer);

        cases++;
        if (case_set.size() > 0 && case_set.find(cases - 1) == case_set.end())
            continue;

        cout << "  Testcase #" << cases - 1 << " ... ";
        if ( do_test(t1, t2, __answer)) {
            passed++;
        }
    }
    if (mainProcess) {
        cout << endl << "Passed : " << passed << "/" << cases << " cases" << endl;
        int T = time(NULL) - 1521302406;
        double PT = T / 60.0, TT = 75.0;
        cout << "Time   : " << T / 60 << " minutes " << T % 60 << " secs" << endl;
        cout << "Score  : " << 250 * (0.3 + (0.7 * TT * TT) / (10.0 * PT * PT + TT * TT)) << " points" << endl;
    }
    return 0;
}

int main(int argc, char *argv[]) {
    cout.setf(ios::fixed, ios::floatfield);
    cout.precision(2);
    set<int> cases;
    bool mainProcess = true;
    for (int i = 1; i < argc; ++i) {
        if ( string(argv[i]) == "-") {
            mainProcess = false;
        } else {
            cases.insert(atoi(argv[i]));
        }
    }
    if (mainProcess) {
        cout << "TreesAndBrackets (250 Points)" << endl << endl;
    }
    return run_test(mainProcess, cases, argv[0]);
}
// CUT end
