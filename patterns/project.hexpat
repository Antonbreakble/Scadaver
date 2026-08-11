#pragma endian little

import std.string;
import std.time;

// TDateTime из Delphi хранит время в double... 
// https://docwiki.embarcadero.com/Libraries/Sydney/en/System.TDateTime.Create

fn format_tdatetime(double value) {
    std::time::EpochTime timestamp =
        s64((value - 25569.0) * 86400.0);

    return std::time::format(
        std::time::to_local(timestamp),
        "%Y-%m-%d %H:%M:%S"
    );
};

using TDateTime = double [[format("format_tdatetime")]];

struct ProjectVersion{
    u16 major;
    u16 minor;
    u16 path;
    u16 build;
};

struct Date{
    u8 day;
    u8 mount;
    u16 year;
};

struct ProjectFile {
    TDateTime create_at;
    
    ProjectVersion version;
    
    Date version_date;
    
    // 2.7.3.1 поля нет
    // 2.7.5.1 поле есть
    u32 version_code; 
    
    u8 unknown[2];
    
    std::string::SizedString<u32> project_name;
};

ProjectFile spr @ 0x00;