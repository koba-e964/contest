#include <iostream>
#include <algorithm>
#include <cstdio>
#include <string>

using namespace std;

int main(void){
	string cur="kogakubu10gokan";
	int n,q;
	cin>>n>>q;
	for(int i=0;i<n;i++){
		int year;
		string name;
		cin>>year>>name;
		if(year<=q){
			cur=name;
		}
	}
	cout<<cur<<endl;
	return 0;
}