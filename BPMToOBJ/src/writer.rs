// Copyright (c) 2021, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::io::Result;
use std::io::Write;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use super::vec::Vec3f;
use super::vec::Vec2f;
use super::decompiler::Triangle;

fn write_vec3f(prefix: &str, writer: &mut dyn Write, vp: &Vec<Vec3f>) -> Result<()>
{
    for v in vp
    {
        writeln!(writer, "{} {} {} {}", prefix, v.x, v.y, v.z)?;
    }
    return Ok(());
}

fn write_vec2f(prefix: &str, writer: &mut dyn Write, vp: &Vec<Vec2f>) -> Result<()>
{
    for v in vp
    {
        writeln!(writer, "{} {} {}", prefix, v.x, v.y)?;
    }
    return Ok(());
}

fn write_tris(writer: &mut dyn Write, tris: &Vec<Triangle>) -> Result<()>
{
    for tri in tris
    {
        writeln!(writer, "f {}/{}/{} {}/{}/{} {}/{}/{}",
            tri.p1.vp + 1, tri.p1.vt + 1, tri.p1.vn + 1, //Blender does not accept the standard 0-based indexing
            tri.p2.vp + 1, tri.p2.vt + 1, tri.p2.vn + 1,
            tri.p3.vp + 1, tri.p3.vt + 1, tri.p3.vn + 1
        )?;
    }
    return Ok(());
}

pub fn write(out: &str, vp: &Vec<Vec3f>, vn: &Vec<Vec3f>, vt: &Vec<Vec2f>, tris: &Vec<Triangle>) -> Result<()>
{
    let file = File::create(Path::new(out))?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "# Generated by BPM Decompiler {}", super::VERSION)?;

    writeln!(writer, "")?;
    writeln!(writer, "# Vertex positions")?;
    write_vec3f("v", &mut writer, &vp)?;

    writeln!(writer, "")?;
    writeln!(writer, "# Vertex normals")?;
    write_vec3f("vn", &mut writer, &vn)?;

    writeln!(writer, "")?;
    writeln!(writer, "# Vertex texture coordinates")?;
    write_vec2f("vt", &mut writer, &vt)?;

    writeln!(writer, "")?;
    writeln!(writer, "# Faces / triangles")?;
    write_tris(&mut writer, &tris)?;

    return Ok(());
}
